```mermaid
 classDiagram

 class Line {
     <<embedded_graphics>>
     into_styled(style) Styled
     
 }

 class Rectangle {
     <<embedded_graphics>>
     into_styled(style) Styled
 }

 class Styled {
     <<embedded_graphics>>
     draw(&display:.&DrawTarget) Result
     
 }

 Line -- Styled
 Rectangle -- Styled
 Styled -- DrawTarget

 class Drawable {
     <<embedded_graphics>>
     draw(&display: &DrawTarget)  Result
 }

 Styled --> Drawable : implements
 
 
class DrawTarget {
  <<embedded_graphics_core::trait>>
  draw_iter(pixels:IntoIterator)* Result
  fill_contiguous(area:&Rectangle, colors:IntoIterator)* Result
  fill_solid(area:&Rectangle, color:Color)*
  clear(color:Color)* Result

}

DrawTarget-->WriteOnlyDataCommand : uses

class WriteOnlyDataCommand {
    <<display_interface::trait>>

    send_command(cmd:DataFormat)* Result
    send_data(buf:DataFormat)* Result
}

class PGPIO8BitInterface {
    send_command(cmd:DataFormat) Result
    send_data(buf:DataFormat) Result
}

WriteOnlyDataCommand  <|-- PGPIO8BitInterface  : implements



```

# Notes 
`PGPIO8BitInterface` is for the 8080 style interface to the display and is a [crate](https://docs.rs/display-interface-parallel-gpio/latest/display_interface_parallel_gpio/struct.PGPIO8BitInterface.html) in its own right