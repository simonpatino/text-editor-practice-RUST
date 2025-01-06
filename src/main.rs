mod editor;

use editor::Editor;

use editor::draw_rows;



fn main() {

    draw_rows();

    Editor::default().run();

}

 


