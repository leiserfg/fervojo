#let rr-wasm = plugin("./typst_railroad.wasm")

#let as_bytes(input) = {
    let t = type(input)
    if t == "bytes" {
        return input
    }
    if t == "string"{
        return bytes(input)
    }
    if input.has("text") {
        return as_bytes(input.text)
    }
    assert(true, message:  "The input is not string, bytes, or a content with text")
}

#let to_svg(
    railroad,
    css: rr-wasm.default_css(),
) = {
    rr-wasm.railroad(as_bytes(railroad), as_bytes(css))
}


#let render(
    railroad,
    css: rr-wasm.default_css(),
    ..args
) = {
    image.decode(to_svg(railroad, css: css), format: "svg", ..args)
}



