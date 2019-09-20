enum HttpCodes
{
    Correct = 200,
    NotFound = 404
}

fn main() {
   let code_correct = HttpCodes::Correct;
   let code_err = HttpCodes::NotFound;
}
