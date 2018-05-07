


fn nextc() -> Result<Char> {

}

fn skip_comment() {
    c = nextc();
    while (c != '\n') {
        c = nextc();
    }
}
