use hilen::App;
use nome::NomeApp;

hilen::register_app!(NomeApp);

fn main() {
    NomeApp::start();
}
