//! Base traits and functionalities for method (calls) and property codecs.

use std::io::{self, Write, Read};

use crate::net::bundle::{BundleElementWriter, TopElementReader, BundleElement, BundleResult};
use crate::util::io::*;

use super::{ElementLength, ElementIdRange, Element, TopElement};


/// A trait to be implemented on enumerations of method calls.
pub trait MethodCall: Sized {

    /// Return the total number of exposed methods for this type of methods.
    fn count() -> u16;

    /// Return the index of the method.
    fn index(&self) -> u16;

    /// Return the length for a given method index.
    fn len(index: u16) -> ElementLength;

    /// Encode the method with the given writer.
    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    /// Decode the method with the given reader, length and for a specific index.
    fn decode(read: &mut impl Read, len: usize, index: u16) -> io::Result<Self>;

}


/// Trait to implement on extension elements that is used together with
/// [`MethodCallWrapper`] to encode or decode generic method call elements.
pub trait MethodCallExt: TopElement<Config = ()> {
    const ID_RANGE: ElementIdRange;
}


/// This special bundle's element can be used to call a method either on
/// the server or on the client. Its interface is quite complicated, so
/// it's only internal and must be used through public functions.
pub struct MethodCallWrapper<M, P>
where
    M: MethodCall,
    P: MethodCallExt,
{
    /// The actual wrapped method call.
    pub method: M,
    /// The extension element that is encoded or decoded just before the
    /// actual method call.
    pub ext: P,
}

impl<M, P> MethodCallWrapper<M, P>
where
    M: MethodCall,
    P: MethodCallExt,
{

    /// The length type to use if now particular length is required, this length 
    /// defaults to a callback which returns a length depending on the id and the
    /// use of sub-slot to represent the method's id.
    pub const DEFAULT_LEN: ElementLength = ElementLength::Callback(|id| {
        
        // This element's length is determined by this callback, so we are relying
        // on the element's id. If this id is a full-slot, we get the length of the
        // method call, but if a sub-slot is needed for encoding the sub-id, the 
        // length of the element is always 16-bits variable.
        if let Some(exposed_id) = P::ID_RANGE.to_exposed_id_checked(M::count(), id) {
            M::len(exposed_id)
        } else {
            ElementLength::Variable16
        }

    });

    pub fn new(method: M, prefix: P) -> Self {
        Self { method, ext: prefix }
    }

    pub fn write(self, mut writer: BundleElementWriter) {

        let (
            element_id, 
            sub_id
        ) = P::ID_RANGE.from_exposed_id(M::count(), self.method.index());
    
        writer.write(element_id, self, &(0, sub_id));

    }

    pub fn read(reader: TopElementReader) -> BundleResult<BundleElement<Self>> {
        let element_id = reader.id();
        reader.read::<Self>(&(element_id, None))
    }

}

impl<M, P> Element for MethodCallWrapper<M, P>
where
    M: MethodCall,
    P: MethodCallExt,
{

    /// This contains (elt_id, sub_id):
    /// - Element id is required when decoding;
    /// - Sub id optional when encoding.
    type Config = (u8, Option<u8>);

    fn encode(&self, write: &mut impl Write, config: &Self::Config) -> io::Result<()> {

        self.ext.encode(write, &())?;

        // Write the sub-id if required.
        if let Some(sub_id) = config.1 {
            write.write_u8(sub_id)?;
        }

        self.method.encode(write)

    }

    fn decode(read: &mut impl Read, mut len: usize, config: &Self::Config) -> io::Result<Self> {

        // Decode the prefix with a counter reader in order to adjust the length afterward.
        let mut prefix_read = IoCounter::new(&mut *read);
        let prefix = P::decode(&mut prefix_read, len, &())?;
        len -= prefix_read.count();

        let mut sub_id_err = None;
        let exposed_id = P::ID_RANGE.to_exposed_id(M::count(), config.0, || {
            // Sub-id is needed.
            len -= 1;
            match read.read_u8() {
                Ok(n) => n,
                Err(e) => {
                    sub_id_err = Some(e);
                    0  // Return zero, this value will not be used anyway.
                }
            }
        });

        // Propagate error.
        if let Some(e) = sub_id_err {
            return Err(e);
        }

        M::decode(read, len, exposed_id).map(|method| Self { 
            method, 
            ext: prefix,
        })

    }

}

impl<M, P> TopElement for MethodCallWrapper<M, P>
where
    M: MethodCall,
    P: MethodCallExt,
{
    const LEN: ElementLength = P::LEN;
}
