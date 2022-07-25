use identicon_rs::Identicon;
use rustler::{OwnedBinary, Binary, NifResult, Env};

rustler::init!(
    "Elixir.NifIdenticon",
    [
        generate_base64,
        generate_binary
    ],
    load = load
);

fn load(_env: rustler::Env, _: rustler::Term) -> bool {
    true
}

#[rustler::nif(name = "generate_binary")]
fn generate_binary<'a>(env: Env<'a>, _name: String) -> NifResult<Binary<'a>> {
    let identicon_vector = Identicon::new(_name).export_png_data().unwrap();
    let identicon_bytes = &identicon_vector[..];
    let mut binary = OwnedBinary::new(identicon_bytes.len()).unwrap();
    binary.as_mut_slice().copy_from_slice(&identicon_bytes);
    return Ok(Binary::from_owned(binary, env));
}


#[rustler::nif(name = "generate_base64")]
fn generate_base64(_name: String) -> NifResult<String> {
    let identicon_vector = Identicon::new(_name).export_png_data().unwrap();
    return Ok(base64::encode(identicon_vector));
}