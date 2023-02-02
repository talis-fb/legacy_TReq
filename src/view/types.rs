

// **Optional talvez**
// Visto que possivelmente sera melhor implementar views que facam cada uma das funções dai
trait Tui<T> {
    fn render_text(&mut self, text: &str, area: T);
    fn render_button(&mut self, label: &str, area: T);
}


trait Backend : Tui {
    // Vai chamar o Component.render passando oq for necessário do back end,
    fn draw(&mut self, view: &Component) -> () {
        view.render(self)
    }
}

struct BackendTuiRs {
}


trait Component<IArea> {
    fn render(&self, f: &mut impl Backend);
}


#[derive(Default)]
struct TextView { 
    text: String,
    rect: Rect---
}

impl Component for TextView {
    fn render(&self, f: &mut impl Backend) {
        f.render_text(self.text)
    }
}
