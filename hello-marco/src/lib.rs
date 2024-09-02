/* A Marco Polog Game

If the name Marco is given, the program will respond with Polo.
Otherwise, the progra mwill erspond with "What's your name?"
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        return String::from("Polo");
    } else {
        return String::from("What's your name?");
    }
}
