// Do not edit: This code was generated by flatdata's generator.
pub mod n {

pub mod schema {
pub mod structs {
}

pub mod x {

pub const X: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

"#;

pub mod resources {
pub const PAYLOAD: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

"#;
}
}
pub mod a {

pub const A: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

namespace n {
archive A
{
    data : archive .n.X;
    @optional
    optional_data : archive .n.X;
}
}

"#;

pub mod resources {
pub const DATA: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

namespace n {
archive A
{
    data : archive .n.X;
}
}

"#;
pub const OPTIONAL_DATA: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

namespace n {
archive A
{
    @optional
    optional_data : archive .n.X;
}
}

"#;
}
}
}


#[derive(Clone)]
pub struct X {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    payload: flatdata::MemoryDescriptor,
}

impl X {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn payload(&self) -> flatdata::RawData {
        flatdata::RawData::new(unsafe {self.payload.as_bytes()})
    }

}

impl ::std::fmt::Debug for X {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("X")
            .field("payload", &self.payload())
            .finish()
    }
}

impl flatdata::Archive for X {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let payload = Self::read_resource(&*storage, "payload", schema::x::resources::PAYLOAD)?;

        Ok(Self {
            _storage: storage,
            payload,
        })
    }
}

#[derive(Clone, Debug)]
pub struct XBuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl XBuilder {
    #[inline]
    pub fn set_payload(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("payload", schema::x::resources::PAYLOAD, data)
    }

}impl flatdata::ArchiveBuilder for XBuilder {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}




#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data: super::n::X,
    optional_data: Option<super::n::X>,
}

impl A {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &super::n::X
    {
        &self.data
    }

    #[inline]
    pub fn optional_data(&self) -> Option<&super::n::X>
    {
        self.optional_data.as_ref()
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("data", &self.data())
            .field("optional_data", &self.optional_data())
            .finish()
    }
}

impl flatdata::Archive for A {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let data = super::n::X::open(storage.subdir("data"))?;
        let optional_data = super::n::X::open(storage.subdir("optional_data")).ok();

        Ok(Self {
            _storage: storage,
            data,
            optional_data,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ABuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl ABuilder {
    #[inline]
    pub fn data(&self) -> Result<super::n::XBuilder, flatdata::ResourceStorageError> {
        use flatdata::ArchiveBuilder;
        let storage = self.storage.subdir("data");
        super::n::XBuilder::new(storage)
    }

    #[inline]
    pub fn optional_data(&self) -> Result<super::n::XBuilder, flatdata::ResourceStorageError> {
        use flatdata::ArchiveBuilder;
        let storage = self.storage.subdir("optional_data");
        super::n::XBuilder::new(storage)
    }

}impl flatdata::ArchiveBuilder for ABuilder {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}

}
