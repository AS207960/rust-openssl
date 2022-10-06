use EVP_PKEY;
use ::{ASN1_OBJECT, ASN1_OCTET_STRING};
use libc::*;
use *;

pub enum CMS_ContentInfo {}
pub enum CMS_SignerIdentifier {}

#[repr(C)]
pub struct CMS_SignerInfo {
    pub version: c_long,
    pub sid: *mut CMS_SignerIdentifier,
    pub digest_algorithm: *mut X509_ALGOR,
    pub signed_attributes: *mut stack_st_X509_ATTRIBUTE,
    pub signature_algorithm: *mut X509_ALGOR,
    pub signature: *mut ASN1_OCTET_STRING,
    pub unsigned_attributes: *mut stack_st_X509_ATTRIBUTE,
    pub signer: *mut X509,
    pub pkey: *mut EVP_PKEY,
}

extern "C" {
    #[cfg(ossl101)]
    pub fn CMS_ContentInfo_free(cms: *mut ::CMS_ContentInfo);
    #[cfg(ossl101)]
    pub fn i2d_CMS_ContentInfo(a: *mut ::CMS_ContentInfo, pp: *mut *mut c_uchar) -> c_int;

    #[cfg(ossl101)]
    pub fn d2i_CMS_ContentInfo(
        a: *mut *mut ::CMS_ContentInfo,
        pp: *mut *const c_uchar,
        length: c_long,
    ) -> *mut ::CMS_ContentInfo;
}

#[cfg(ossl101)]
pub const CMS_TEXT: c_uint = 0x1;
#[cfg(ossl101)]
pub const CMS_NOCERTS: c_uint = 0x2;
#[cfg(ossl101)]
pub const CMS_NO_CONTENT_VERIFY: c_uint = 0x4;
#[cfg(ossl101)]
pub const CMS_NO_ATTR_VERIFY: c_uint = 0x8;
#[cfg(ossl101)]
pub const CMS_NOSIGS: c_uint = 0x4 | 0x8;
#[cfg(ossl101)]
pub const CMS_NOINTERN: c_uint = 0x10;
#[cfg(ossl101)]
pub const CMS_NO_SIGNER_CERT_VERIFY: c_uint = 0x20;
#[cfg(ossl101)]
pub const CMS_NOVERIFY: c_uint = 0x20;
#[cfg(ossl101)]
pub const CMS_DETACHED: c_uint = 0x40;
#[cfg(ossl101)]
pub const CMS_BINARY: c_uint = 0x80;
#[cfg(ossl101)]
pub const CMS_NOATTR: c_uint = 0x100;
#[cfg(ossl101)]
pub const CMS_NOSMIMECAP: c_uint = 0x200;
#[cfg(ossl101)]
pub const CMS_NOOLDMIMETYPE: c_uint = 0x400;
#[cfg(ossl101)]
pub const CMS_CRLFEOL: c_uint = 0x800;
#[cfg(ossl101)]
pub const CMS_STREAM: c_uint = 0x1000;
#[cfg(ossl101)]
pub const CMS_NOCRL: c_uint = 0x2000;
#[cfg(ossl101)]
pub const CMS_PARTIAL: c_uint = 0x4000;
#[cfg(ossl101)]
pub const CMS_REUSE_DIGEST: c_uint = 0x8000;
#[cfg(ossl101)]
pub const CMS_USE_KEYID: c_uint = 0x10000;
#[cfg(ossl101)]
pub const CMS_DEBUG_DECRYPT: c_uint = 0x20000;
#[cfg(ossl102)]
pub const CMS_KEY_PARAM: c_uint = 0x40000;
#[cfg(ossl110)]
pub const CMS_ASCIICRLF: c_uint = 0x80000;
#[cfg(ossl111)]
pub const CMS_CADES: c_uint = 0x100000;

extern "C" {
    #[cfg(ossl101)]
    pub fn SMIME_read_CMS(bio: *mut ::BIO, bcont: *mut *mut ::BIO) -> *mut ::CMS_ContentInfo;

    #[cfg(ossl101)]
    pub fn CMS_sign(
        signcert: *mut ::X509,
        pkey: *mut ::EVP_PKEY,
        certs: *mut ::stack_st_X509,
        data: *mut ::BIO,
        flags: c_uint,
    ) -> *mut ::CMS_ContentInfo;

    #[cfg(ossl101)]
    pub fn CMS_encrypt(
        certs: *mut stack_st_X509,
        data: *mut ::BIO,
        cipher: *const EVP_CIPHER,
        flags: c_uint,
    ) -> *mut ::CMS_ContentInfo;

    #[cfg(ossl101)]
    pub fn CMS_decrypt(
        cms: *mut ::CMS_ContentInfo,
        pkey: *mut ::EVP_PKEY,
        cert: *mut ::X509,
        dcont: *mut ::BIO,
        out: *mut ::BIO,
        flags: c_uint,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_add0_cert(
        cms: *mut ::CMS_ContentInfo,
        cert: *mut ::X509,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_add1_signer(
        cms: *mut ::CMS_ContentInfo,
        cert: *mut ::X509,
        pkey: *mut ::EVP_PKEY,
        md: *const EVP_MD,
        flags: c_uint,
    ) -> *mut ::CMS_SignerInfo;

    #[cfg(ossl101)]
    pub fn CMS_final(
        cms: *mut ::CMS_ContentInfo,
        data: *mut ::BIO,
        dcont: *mut ::BIO,
        flags: c_uint,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_signed_add1_attr_by_OBJ(
        si: *mut CMS_SignerInfo,
        obj: *const ASN1_OBJECT,
		attr_type: c_int,
		bytes: *const c_uchar,
		len: c_int,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_signed_add1_attr_by_NID(
        si: *mut CMS_SignerInfo,
		nid: c_int,
		attr_type: c_int,
		bytes: *const c_uchar,
		len: c_int,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_unsigned_add1_attr_by_OBJ(
        si: *mut CMS_SignerInfo,
        obj: *const ASN1_OBJECT,
		attr_type: c_int,
		bytes: *const c_uchar,
		len: c_int,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_unsigned_add1_attr_by_NID(
        si: *mut CMS_SignerInfo,
		nid: c_int,
		attr_type: c_int,
		bytes: *const c_uchar,
		len: c_int,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_signed_get_attr_count(
        si: *const CMS_SignerInfo,
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_signed_get_attr_by_NID(
        si: *const CMS_SignerInfo,
        nid: c_int,
        lastpos: c_int
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_signed_get_attr_by_OBJ(
        si: *const CMS_SignerInfo,
        obj: *mut ASN1_OBJECT,
        lastpos: c_int
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_signed_delete_attr(
        si: *mut CMS_SignerInfo,
		loc: c_int
	) -> *mut X509_ATTRIBUTE;

    #[cfg(ossl101)]
    pub fn CMS_unsigned_get_attr_by_NID(
        si: *const CMS_SignerInfo,
        nid: c_int,
        lastpos: c_int
    ) -> c_int;

    #[cfg(ossl101)]
    pub fn CMS_unsigned_delete_attr(
        si: *mut CMS_SignerInfo,
		loc: c_int
	) -> *mut X509_ATTRIBUTE;

    #[cfg(ossl101)]
    pub fn CMS_SignerInfo_sign(
        si: *mut CMS_SignerInfo,
	) -> c_int;
}
