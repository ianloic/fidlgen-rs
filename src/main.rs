#[macro_use]
extern crate serde_derive;

use std::env;

mod ir {

    use std::collections::HashMap;
    use std::fs::File;

    type Ordinal = u64;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    enum HandleSubtype {
        Handle,
        Process,
        Thread,
        VMO,
        Channel,
        Event,
        Port,
        Interrupt,
        DebugLog,
        Socket,
        Resource,
        EventPair,
        Job,
        VMAR,
        FIFO,
        Guest,
        Timer,
        BTI,
        Profile,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    enum DeclarationType {
        Const,
        Bits,
        Enum,
        Interface,
        Struct,
        Table,
        Union,
        XUnion,
    }

    #[derive(Deserialize, Debug)]
    #[serde(tag = "kind")]
    #[serde(rename_all = "lowercase")]
    enum Type {
        Array {
            element_type: Box<Type>,
            element_count: u32,
        },
        Vector {
            element_type: Box<Type>,
            nullable: bool,
            maybe_element_count: Option<u32>,
        },
        r#String {
            nullable: bool,
            maybe_element_count: Option<u32>,
        },
        Handle {
            subtype: HandleSubtype,
            nullable: bool,
        },
        Request {
            subtype: String,
            nullable: bool,
        },
        Primitive {
            subtype: String,
        },
        Identifier {
            identifier: String,
            nullable: bool,
        },
    }

    #[derive(Deserialize, Debug)]
    #[serde(tag = "kind")]
    #[serde(rename_all = "lowercase")]
    enum Literal {
        r#String { value: String },
        Numeric { value: String },
        True {},
        False {},
        r#Default {},
    }

    #[derive(Deserialize, Debug)]
    #[serde(tag = "kind")]
    #[serde(rename_all = "lowercase")]
    enum Constant {
        Identifier { identifier: String },
        Literal { literal: Literal },
    }
    #[derive(Deserialize, Debug)]
    struct Attribute {
        name: String,
        value: String,
    }

    #[derive(Deserialize, Debug)]
    struct Const {
        name: String,
        r#type: Type,
        value: Constant,
        maybe_attributes: Option<Vec<Attribute>>,
    }
    #[derive(Deserialize, Debug)]
    struct EnumMember {
        name: String,
        value: Constant,
        maybe_attributes: Option<Vec<Attribute>>,
    }
    #[derive(Deserialize, Debug)]
    struct Enum {
        name: String,
        r#type: String,
        members: Vec<EnumMember>,
        maybe_attributes: Option<Vec<Attribute>>,
    }

    #[derive(Deserialize, Debug)]
    struct ProtocolMethod {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
        ordinal: Ordinal,
        generated_ordinal: Ordinal,
        has_request: bool,
        maybe_request: Option<Vec<StructMember>>,
        maybe_request_size: Option<u32>,
        maybe_request_alignment: Option<u32>,
        has_response: bool,
        maybe_response: Option<Vec<StructMember>>,
        maybe_response_size: Option<u32>,
        maybe_response_alignment: Option<u32>,
    }

    #[derive(Deserialize, Debug)]
    struct Protocol {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
        methods: Vec<ProtocolMethod>,
    }

    #[derive(Deserialize, Debug)]
    struct StructMember {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
        r#type: Type,
        size: u32,
        max_out_of_line: u32,
        alignment: u32,
        offset: u32,
        maybe_default_value: Option<Constant>,
    }

    #[derive(Deserialize, Debug)]
    struct Struct {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
        members: Vec<StructMember>,
        size: u32,
        max_out_of_line: u32,
        max_handles: Option<u32>,
        anonymous: Option<bool>,
    }

    #[derive(Deserialize, Debug)]
    struct TableMember {
        // TODO: this should be an enum of reserved and a the actual declaration
        reserved: bool,
        ordinal: Ordinal,
        name: Option<String>,
        r#type: Option<Type>,
        size: Option<u32>,
        max_out_of_line: Option<u32>,
        alignment: Option<u32>,
        offset: Option<u32>,
        maybe_default_value: Option<Constant>,
    }

    #[derive(Deserialize, Debug)]
    struct Table {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
        members: Vec<TableMember>,
        size: u32,
        max_out_of_line: u32,
    }

    #[derive(Deserialize, Debug)]
    struct UnionMember {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
        r#type: Type,
        size: u32,
        max_out_of_line: u32,
        alignment: u32,
        offset: u32,
    }
    #[derive(Deserialize, Debug)]
    struct Union {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
        members: Vec<UnionMember>,
        size: u32,
        alignment: u32,
        max_out_of_line: u32,
        max_handles: Option<u32>,
    }

    #[derive(Deserialize, Debug)]
    struct Declaration {
        name: String,
        maybe_attributes: Option<Vec<Attribute>>,
    }

    #[derive(Deserialize, Debug)]
    struct LibraryDependency {
        name: String,
        declarations: HashMap<String, DeclarationType>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Library {
        version: String,
        name: String,
        const_declarations: Vec<Const>,
        enum_declarations: Vec<Enum>,
        interface_declarations: Vec<Protocol>,
        struct_declarations: Vec<Struct>,
        table_declarations: Vec<Table>,
        union_declarations: Vec<Union>,
        xunion_declarations: Vec<Declaration>,
        declaration_order: Vec<String>,
        declarations: HashMap<String, DeclarationType>,
        library_dependencies: Vec<LibraryDependency>,
    }

    pub fn read_ir(filepath: &str) -> Library {
        serde_json::from_reader(File::open(filepath).expect("file not found")).expect("json error")
    }

}

fn main() {
    for f in env::args().skip(1) {
        let lib = ir::read_ir(&f);
        println!("{:#?}", lib);
    }
}
