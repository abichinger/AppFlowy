// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `view_update.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct UpdateViewRequest {
    // message fields
    pub view_id: ::std::string::String,
    // message oneof groups
    pub one_of_name: ::std::option::Option<UpdateViewRequest_oneof_one_of_name>,
    pub one_of_desc: ::std::option::Option<UpdateViewRequest_oneof_one_of_desc>,
    pub one_of_thumbnail: ::std::option::Option<UpdateViewRequest_oneof_one_of_thumbnail>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UpdateViewRequest {
    fn default() -> &'a UpdateViewRequest {
        <UpdateViewRequest as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateViewRequest_oneof_one_of_name {
    name(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateViewRequest_oneof_one_of_desc {
    desc(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateViewRequest_oneof_one_of_thumbnail {
    thumbnail(::std::string::String),
}

impl UpdateViewRequest {
    pub fn new() -> UpdateViewRequest {
        ::std::default::Default::default()
    }

    // string view_id = 1;


    pub fn get_view_id(&self) -> &str {
        &self.view_id
    }
    pub fn clear_view_id(&mut self) {
        self.view_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_view_id(&mut self, v: ::std::string::String) {
        self.view_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_view_id(&mut self) -> &mut ::std::string::String {
        &mut self.view_id
    }

    // Take field
    pub fn take_view_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.view_id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_name(&mut self) {
        self.one_of_name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.one_of_name = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(_)) = self.one_of_name {
        } else {
            self.one_of_name = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(::std::string::String::new()));
        }
        match self.one_of_name {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        if self.has_name() {
            match self.one_of_name.take() {
                ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string desc = 3;


    pub fn get_desc(&self) -> &str {
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_desc(&mut self) {
        self.one_of_desc = ::std::option::Option::None;
    }

    pub fn has_desc(&self) -> bool {
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.one_of_desc = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(v))
    }

    // Mutable pointer to the field.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(_)) = self.one_of_desc {
        } else {
            self.one_of_desc = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(::std::string::String::new()));
        }
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        if self.has_desc() {
            match self.one_of_desc.take() {
                ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string thumbnail = 4;


    pub fn get_thumbnail(&self) -> &str {
        match self.one_of_thumbnail {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_thumbnail(&mut self) {
        self.one_of_thumbnail = ::std::option::Option::None;
    }

    pub fn has_thumbnail(&self) -> bool {
        match self.one_of_thumbnail {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_thumbnail(&mut self, v: ::std::string::String) {
        self.one_of_thumbnail = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(v))
    }

    // Mutable pointer to the field.
    pub fn mut_thumbnail(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(_)) = self.one_of_thumbnail {
        } else {
            self.one_of_thumbnail = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(::std::string::String::new()));
        }
        match self.one_of_thumbnail {
            ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_thumbnail(&mut self) -> ::std::string::String {
        if self.has_thumbnail() {
            match self.one_of_thumbnail.take() {
                ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for UpdateViewRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.view_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_name = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_name::name(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_desc = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_desc::desc(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_thumbnail = ::std::option::Option::Some(UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(is.read_string()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.view_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.view_id);
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateViewRequest_oneof_one_of_name::name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_desc {
            match v {
                &UpdateViewRequest_oneof_one_of_desc::desc(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_thumbnail {
            match v {
                &UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.view_id.is_empty() {
            os.write_string(1, &self.view_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateViewRequest_oneof_one_of_name::name(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_desc {
            match v {
                &UpdateViewRequest_oneof_one_of_desc::desc(ref v) => {
                    os.write_string(3, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_thumbnail {
            match v {
                &UpdateViewRequest_oneof_one_of_thumbnail::thumbnail(ref v) => {
                    os.write_string(4, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UpdateViewRequest {
        UpdateViewRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "view_id",
                |m: &UpdateViewRequest| { &m.view_id },
                |m: &mut UpdateViewRequest| { &mut m.view_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "name",
                UpdateViewRequest::has_name,
                UpdateViewRequest::get_name,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "desc",
                UpdateViewRequest::has_desc,
                UpdateViewRequest::get_desc,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "thumbnail",
                UpdateViewRequest::has_thumbnail,
                UpdateViewRequest::get_thumbnail,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UpdateViewRequest>(
                "UpdateViewRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UpdateViewRequest {
        static instance: ::protobuf::rt::LazyV2<UpdateViewRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UpdateViewRequest::new)
    }
}

impl ::protobuf::Clear for UpdateViewRequest {
    fn clear(&mut self) {
        self.view_id.clear();
        self.one_of_name = ::std::option::Option::None;
        self.one_of_desc = ::std::option::Option::None;
        self.one_of_thumbnail = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateViewRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateViewRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11view_update.proto\"\xaa\x01\n\x11UpdateViewRequest\x12\x17\n\x07vi\
    ew_id\x18\x01\x20\x01(\tR\x06viewId\x12\x14\n\x04name\x18\x02\x20\x01(\t\
    H\0R\x04name\x12\x14\n\x04desc\x18\x03\x20\x01(\tH\x01R\x04desc\x12\x1e\
    \n\tthumbnail\x18\x04\x20\x01(\tH\x02R\tthumbnailB\r\n\x0bone_of_nameB\r\
    \n\x0bone_of_descB\x12\n\x10one_of_thumbnailJ\xd7\x02\n\x06\x12\x04\0\0\
    \x07\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\
    \x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x19\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x03\x04\x17\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x03\x15\x16\n\x0b\n\x04\x04\0\x08\0\x12\x03\x04\x04*\n\x0c\n\
    \x05\x04\0\x08\0\x01\x12\x03\x04\n\x15\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x04\x18(\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x18\x1e\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x04\x1f#\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x04&'\n\x0b\n\x04\x04\0\x08\x01\x12\x03\x05\x04*\n\x0c\n\x05\x04\0\
    \x08\x01\x01\x12\x03\x05\n\x15\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x18\
    (\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x18\x1e\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03\x05\x1f#\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05&\
    '\n\x0b\n\x04\x04\0\x08\x02\x12\x03\x06\x044\n\x0c\n\x05\x04\0\x08\x02\
    \x01\x12\x03\x06\n\x1a\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x06\x1d2\n\x0c\
    \n\x05\x04\0\x02\x03\x05\x12\x03\x06\x1d#\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x03\x06$-\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0601b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}