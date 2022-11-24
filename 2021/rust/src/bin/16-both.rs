use std::{collections::HashMap, rc::Rc};

const WORD_LENGTH: usize = 128;
fn main() {
    let file = std::fs::read_to_string("./16-input.txt").unwrap();
    let data = file.lines().next().unwrap();
    // println!("Data: \"{}\"", data);
    println!("Version sum: {}", sum_versions(data));
    // print_packet(data);

    println!("Result: {}", calculate_hex_string(data).unwrap())
}

fn calculate_hex_string(hex_string: &str) -> Result<u128, String> {
    let (cursor_start, byte_array) = parse_hex_string(hex_string);
    let mut reader = Reader::from_slice(&byte_array, cursor_start);
    let mut vec_visitor = CollectVisitor::new();
    read_packet(&mut reader, &mut vec_visitor).unwrap();
    let messages = vec_visitor.get_data();
    let ast = create_ast_from_messages(messages).unwrap();
    Ok(ast.interpret())
}

fn create_ast_from_messages(messages: Vec<Message>) -> Result<Node, String> {
    let mut stack: Vec<Node> = Vec::new();
    let mut current: Option<Node> = None;

    let mut container_stack: Vec<Vec<Node>> = Vec::new();
    let mut current_container: Option<Vec<Node>> = Some(Vec::new());
    for message in messages {
        match message {
            Message::Type(packet_type) => {
                // TODO: Should be if let 
                match packet_type {
                    MessagePacketType::Data => { /* Ignore message, we'll handle it when we receive the data */ },
                    _ => { current = Some(Node::Operator(OperatorPacket::new(packet_type))); }
                }
            },
            Message::Data(data) => { current = Some(Node::Data(data)) }
            Message::PacketEnd => {
                if let Some(node) = current {
                    if let Some(ref mut cur_con) = current_container {
                        cur_con.push(node);
                    } else {
                        return Err("Attempt to push to non-existing container".to_string());
                    }
                }
                current = None
            },
            Message::ContainerStart(_) => {
                if let Some(cur_con) = current_container {
                    container_stack.push(cur_con);
                }
                current_container = Some(Vec::new());

                stack.push(current.unwrap());
                current = None;
            },
            Message::ContainerEnd => {
                current = stack.pop();

                if let Some(ref mut node) = current {
                    if let Node::Operator(ref mut op) = node { 
                        op.children = current_container.unwrap();
                    }
                } else {
                    return Err("No current node to add children to".to_string());
                }

                current_container = container_stack.pop();
            },
            _ => {}
        }
    }
    current_container.ok_or("Something went wrong".to_string())?.pop().ok_or(String::from("Something went terribly wrong"))
}

#[allow(dead_code)]
fn print_packet(data: &str) {
    let (cursor_start, byte_array) = parse_hex_string(data);
    let mut reader = Reader::from_slice(&byte_array, cursor_start);
    read_packet(&mut reader, &mut PrintVisitor{});
}

fn sum_versions(data: &str) -> u32{
    let (cursor_start, byte_array) = parse_hex_string(data);
    let mut version_visitor = VersionSumVisitor { sum: 0 };
    let mut reader = Reader::from_slice(&byte_array, cursor_start);
    let res = read_packet(&mut reader, &mut version_visitor);
    match res{_=>{}}
    version_visitor.sum
}

fn read_packet(reader: &mut Reader, visitor: &mut impl Visitor) -> Result<(), String> {
    visitor.visit(Message::PacketStart);
    let version = reader.read(3)?;
    visitor.visit(Message::Version(version as u8));
    let packet_type = MessagePacketType::from_u8(reader.read(3)? as u8)?;
    visitor.visit(Message::Type(packet_type));

    match packet_type {
        MessagePacketType::Data => read_data_packet(reader, visitor),
        _ => read_operator_packet(reader, visitor)
    }?;

    visitor.visit(Message::PacketEnd);
    Ok(())
}

fn read_data_packet(reader: &mut Reader, visitor: &mut impl Visitor) -> Result<(), String> {
    let mut acc: u128 = 0;
    let data = loop {
        let last_segment = !reader.read_bool()?;
        let cur = reader.read(4)?;

        acc = (acc << 4) + cur as u128;

        if last_segment {
            break acc
        }
    };

    visitor.visit(Message::Data(data));
    Ok(())
}

fn read_operator_packet(reader: &mut Reader, visitor: &mut impl Visitor) -> Result<(), String> {
    let length_wise = ! reader.read_bool()?;
    if length_wise {
        let subpacket_length = reader.read(15)? as usize;
        let start_cursor_position = reader.cursor;
        visitor.visit(Message::ContainerStart(ContainerSize::Length(subpacket_length as u16)));
        while reader.cursor - start_cursor_position < subpacket_length {
            read_packet(reader, visitor)?;
        }
        visitor.visit(Message::ContainerEnd);
    } else {
        let packet_count = reader.read(11)?;
        visitor.visit(Message::ContainerStart(ContainerSize::Count(packet_count)));
        for _i in 0..packet_count {
            read_packet(reader, visitor)?;
        }
        visitor.visit(Message::ContainerEnd);
    }

    Ok(())
}

struct Reader<'a> {
    data: &'a [u128],
    cursor: usize
}


fn parse_hex_string(input: &str) -> (usize, Vec<u128>) {
    let chars = input.chars().map(|x| x.to_digit(16).unwrap() as u8).collect::<Vec<u8>>();
    let byte_array = chars.rchunks(WORD_LENGTH / 4).map(|x|{
        x.iter().fold(0, |acc, &cur| {
            acc * 16 + cur as u128
        })
    }).rev().collect();

    (WORD_LENGTH - ((input.len() * 4) % WORD_LENGTH), byte_array)
}

impl<'a> Reader<'a> {
    fn from_slice(data: &'a [u128], cursor_start: usize) -> Self {
        Self {
            data,
            cursor: cursor_start
        }
    }

    fn read_bool(&mut self) -> Result<bool, String> {
        Ok(self.read(1)? == 1)
    }

    fn read(&mut self, length: usize) -> Result<u16, String> {
        let word_index = self.cursor / WORD_LENGTH;
        let bit_index = self.cursor % WORD_LENGTH;
        if bit_index + length <= WORD_LENGTH {
            if word_index >= self.data.len() {
                return Err("End of stream".to_string())
            }
            let output = (self.data[word_index] << bit_index) >> (WORD_LENGTH - length);
            self.cursor += length;
            Ok(output as u16)
        } else {
            if word_index + 1 >= self.data.len() {
                return Err("End of stream".to_string())
            }
            let length_till_boundary = WORD_LENGTH - bit_index;
            let remaining_length = length - length_till_boundary;
            let msb = self.read(length_till_boundary)?;
            let lsb = self.read(remaining_length)?;
            let output = (msb << remaining_length) + lsb;

            Ok(output)
        }
    }
}

trait Visitor {
    fn visit(&mut self, message: Message);
}
struct PrintVisitor {}
impl Visitor for PrintVisitor {
    fn visit(&mut self, message: Message) {
        println!("{:?}", message);
    }
}

#[derive(Debug)]
enum Node {
    Operator(OperatorPacket),
    Data(u128)
}

impl Node {
    fn interpret(&self) -> u128 {
        match self {
            Node::Operator(packet) => packet.interpret(),
            Node::Data(val) => *val,
        }
    }
}

#[derive(Debug)]
struct OperatorPacket {
    operator_type: MessagePacketType,
    children: Vec<Node>
}

impl OperatorPacket {
    fn new(operator_type: MessagePacketType) -> Self {
        Self { operator_type, children: Vec::new() }
    }

    fn interpret(&self) -> u128 {
        let mut values = self.children.iter().map(|node| {node.interpret()} );
        match self.operator_type {
            MessagePacketType::Sum => { values.sum() },
            MessagePacketType::Product => { values.product() },
            MessagePacketType::Minimum => { values.min().unwrap() },
            MessagePacketType::Maximum => { values.max().unwrap() },
            MessagePacketType::Greater => {
                assert!(self.children.len() == 2);
                if values.next().unwrap() > values.next().unwrap() { 1 } else { 0 }
            },
            MessagePacketType::Less => {
                assert!(self.children.len() == 2);
                if values.next().unwrap() < values.next().unwrap() { 1 } else { 0 }
            },
            MessagePacketType::Equal => {
                assert!(self.children.len() == 2);
                if values.next().unwrap() == values.next().unwrap() { 1 } else { 0 }
            },
            _ => { unreachable!() }
        }
    }
}
#[derive(Debug)]
struct VersionSumVisitor { sum: u32 }

impl Visitor for VersionSumVisitor {
    fn visit(&mut self, message: Message) {
        if let Message::Version(version) = message {
            self.sum += version as u32;
        }
    }
}

struct CollectVisitor {
    data: Vec<Message>
}

impl CollectVisitor {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn get_data(self) -> Vec<Message> {
        return self.data;
    }
}

impl Visitor for CollectVisitor {
    fn visit(&mut self, message: Message) {
        self.data.push(message);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Message { Version(u8), Type(MessagePacketType), Data(u128), PacketStart, PacketEnd, ContainerStart(ContainerSize), ContainerEnd }

#[derive(Debug, Hash, PartialEq, Eq)]
enum ContainerSize { Length(u16), Count(u16) }

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum MessagePacketType { Sum, Product, Minimum, Maximum, Data, Greater, Less, Equal }

impl MessagePacketType {
    fn from_u8(type_id: u8) -> Result<Self, String> {
        match type_id {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            4 => Ok(Self::Data),
            5 => Ok(Self::Greater),
            6 => Ok(Self::Less),
            7 => Ok(Self::Equal),
            _ => Err("Unknown packet type encountered".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_hex_string, Reader, PrintVisitor, WORD_LENGTH, read_packet, sum_versions, calculate_hex_string};

    #[test]
    fn test_parse_hex_string() {
        let (cursor, byte_array) = parse_hex_string("0");
        assert_eq!(WORD_LENGTH - 4, cursor);
        assert_eq!(byte_array, vec![0]);

        let (cursor, byte_array) = parse_hex_string("F");
        assert_eq!(WORD_LENGTH - 4, cursor);
        assert_eq!(byte_array, vec![15]);

        let (cursor, byte_array) = parse_hex_string("0F");
        assert_eq!(WORD_LENGTH - 8, cursor);
        assert_eq!(byte_array, vec![15]);

        let (cursor, byte_array) = parse_hex_string("F0");
        assert_eq!(WORD_LENGTH - 8, cursor);
        assert_eq!(byte_array, vec![240]);

        let (cursor, byte_array) = parse_hex_string("FF");
        assert_eq!(WORD_LENGTH - 8, cursor);
        assert_eq!(byte_array, vec![255]);

        let (cursor, byte_array) = parse_hex_string("3FFFFFFFF");
        assert_eq!(WORD_LENGTH - 9*4, cursor);
        assert_eq!(byte_array, vec![17179869183]);
    }

    #[test]
    fn test_reader() {
        let (cursor_start, byte_array) = parse_hex_string("F");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(15, reader.read(4).unwrap());

        let (cursor_start, byte_array) = parse_hex_string("FF");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(15, reader.read(4).unwrap());
        assert_eq!(15, reader.read(4).unwrap());

        let (cursor_start, byte_array) = parse_hex_string("3");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(1, reader.read(3).unwrap());

        let (cursor_start, byte_array) = parse_hex_string("3F");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(1, reader.read(3).unwrap());

        let (cursor_start, byte_array) = parse_hex_string("3FFF");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(1, reader.read(3).unwrap());

        let (cursor_start, byte_array) = parse_hex_string("3FFFFF");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(1, reader.read(3).unwrap());

        let (cursor_start, byte_array) = parse_hex_string("3FFFFFFF");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(1, reader.read(3).unwrap());

        let (cursor_start, byte_array) = parse_hex_string("38006F45291200");
        let mut reader = Reader::from_slice(&byte_array, cursor_start);
        assert_eq!(1, reader.read(3).unwrap());
    }

    #[test]
    fn test_reader_from_slice() {
        // let reader = Reader::from_slice(data, cursor_start)
    }

    #[test]
    fn test_part1_example_input1() {
        let example3= "8A004A801A8002F478";
        assert_eq!(16, sum_versions(example3));
    }

    #[test]
    fn test_part1_example_input2() {
        let example4 = "620080001611562C8802118E34";
        assert_eq!(12, sum_versions(example4));
    }

    #[test]
    fn test_part1_example_input3() {
        let example5 = "C0015000016115A2E0802F182340";
        assert_eq!(23, sum_versions(example5));
    }

    #[test]
    fn test_part1_example_input4() {
        let example6 = "A0016C880162017C3686B18A3D4780";
        assert_eq!(31, sum_versions(example6));
    }

    #[test]
    fn test_first_puzzle() {
        let file = std::fs::read_to_string("./16-input.txt").unwrap();
        let data = file.lines().next().unwrap();
        // assert_eq!(883, sum_versions(data));
    }

    #[test]
    fn test_part2_example_input() {
        assert_eq!(3, calculate_hex_string("C200B40A82").unwrap());
        assert_eq!(54, calculate_hex_string("04005AC33890").unwrap());
        assert_eq!(7, calculate_hex_string("880086C3E88112").unwrap());
        assert_eq!(9, calculate_hex_string("CE00C43D881120").unwrap());
        assert_eq!(1, calculate_hex_string("D8005AC2A8F0").unwrap());
        assert_eq!(0, calculate_hex_string("F600BC2D8F").unwrap());
        assert_eq!(0, calculate_hex_string("9C005AC2F8F0").unwrap());
        assert_eq!(1, calculate_hex_string("9C0141080250320F1802104A08").unwrap());
    }
}