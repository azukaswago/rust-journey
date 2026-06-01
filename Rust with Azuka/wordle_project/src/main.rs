use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug)]
enum LetterResult {
    CorrectPosition,
    WrongPosition,
    NotInWord,
}

fn main() {
    let mut easy_words: HashMap<&str, &str> = HashMap::new();
    easy_words.insert("brave", "ready to face danger without fear");
    easy_words.insert("crisp", "firm and fresh in texture");
    easy_words.insert("blaze", "a large and fiercely burning fire");
    easy_words.insert("grasp", "to take hold of something firmly");
    easy_words.insert("stomp", "to walk with heavy forceful steps");
    easy_words.insert("groan", "a deep sound expressing pain or despair");
    easy_words.insert("scout", "one who searches ahead for information");
    easy_words.insert("pluck", "to pull something sharply away");
    easy_words.insert("stash", "a secret store of something");
    easy_words.insert("gleam", "a small beam of soft light");

    let mut hard_words: HashMap<&str, &str> = HashMap::new();
    hard_words.insert("serendipity", "a happy discovery made by chance");
    hard_words.insert("ephemeral", "existing for only a brief period");
    hard_words.insert("melancholy", "a deep and persistent feeling of sadness");
    hard_words.insert("clandestine", "carried out in secrecy and concealment");
    hard_words.insert(
        "quintessential",
        "the most perfect representative of something",
    );
    hard_words.insert("ambiguous", "open to more than one interpretation");
    hard_words.insert("resilience", "the ability to recover from difficulty");
    hard_words.insert(
        "eloquence",
        "the ability to speak fluently and persuasively",
    );
    hard_words.insert(
        "paradox",
        "a statement that contradicts itself yet may be true",
    );
    hard_words.insert("nostalgia", "a sentimental longing for the past");
    hard_words.insert("labyrinth", "a complex and confusing network of paths");
    hard_words.insert("pragmatic", "dealing with things in a practical way");
    hard_words.insert(
        "cynical",
        "believing people are motivated only by self-interest",
    );
    hard_words.insert(
        "ethereal",
        "extremely delicate and light, almost otherworldly",
    );
    hard_words.insert("catalyst", "something that triggers or speeds up change");
    hard_words.insert("enigma", "a person or thing that is mysterious");
    hard_words.insert("solitude", "the state of being alone");
    hard_words.insert("euphoria", "a feeling of intense happiness and excitement");
    hard_words.insert("tenacious", "holding firmly to something, not giving up");
    hard_words.insert("obsolete", "no longer in use or useful");
    hard_words.insert("perennial", "lasting for a long time or recurring");
    hard_words.insert("profound", "very deep in meaning or intensity");
    hard_words.insert("spontaneous", "happening naturally without planning");
    hard_words.insert("tranquil", "free from disturbance, calm and peaceful");
    hard_words.insert("vehement", "showing strong and intense feeling");
    hard_words.insert(
        "whimsical",
        "playfully quaint or fanciful in an appealing way",
    );
    hard_words.insert("zealous", "having great energy and enthusiasm for a cause");
    hard_words.insert("benevolent", "well meaning and kindly toward others");
    hard_words.insert("complacent", "showing uncritical satisfaction with oneself");
    hard_words.insert("diligent", "having steady and careful effort in work");
    hard_words.insert("empathy", "the ability to understand another's feelings");
    hard_words.insert("frugal", "sparing in use of resources, not wasteful");
    hard_words.insert("gregarious", "fond of company, sociable by nature");
    hard_words.insert("hubris", "excessive pride leading to downfall");
    hard_words.insert(
        "integrity",
        "the quality of being honest and having strong morals",
    );
    hard_words.insert("jovial", "cheerful and friendly in manner");
    hard_words.insert("kinetic", "relating to or resulting from motion");
    hard_words.insert("lethargic", "affected by sluggishness and lack of energy");
    hard_words.insert("meticulous", "showing great attention to detail");
    hard_words.insert("nonchalant", "feeling or appearing casually calm");
    hard_words.insert("obscure", "not discovered or known about by many");
    hard_words.insert("pensive", "engaged in deep or serious thought");
    hard_words.insert("quaint", "attractively unusual or old-fashioned");
    hard_words.insert("reclusive", "avoiding the company of other people");
    hard_words.insert("stoic", "enduring pain without showing feelings");
    hard_words.insert("taciturn", "reserved, saying little");
    hard_words.insert("ubiquitous", "present or appearing everywhere");
    hard_words.insert("verbose", "using more words than needed");
    hard_words.insert("wistful", "having a feeling of vague longing");
    hard_words.insert("xenial", "friendly to strangers and guests");
    hard_words.insert("yearning", "a feeling of intense longing");
    hard_words.insert("arduous", "involving great effort and difficulty");
    hard_words.insert("brevity", "concise and exact use of words");
    hard_words.insert("candor", "the quality of being open and honest");
    hard_words.insert("daunting", "seeming difficult to deal with");
    hard_words.insert("eccentric", "unconventional and slightly strange");
    hard_words.insert("fallacy", "a mistaken belief based on faulty reasoning");
    hard_words.insert("gallant", "brave and heroic in manner");
    hard_words.insert("haughty", "arrogantly superior in manner");
    hard_words.insert("impeccable", "in accordance with the highest standards");
    hard_words.insert("jaded", "tired and lacking enthusiasm");
    hard_words.insert("keen", "having a sharp edge or strong interest");
    hard_words.insert("lucid", "expressed clearly and easy to understand");
    hard_words.insert("mundane", "lacking interest or excitement, ordinary");
    hard_words.insert("nuance", "a subtle difference in meaning or expression");
    hard_words.insert("ominous", "giving the impression of coming danger");
    hard_words.insert("pompous", "affectedly grand and self-important");
    hard_words.insert("quell", "to put an end to a rebellion or feeling");
    hard_words.insert("rancor", "bitterness or resentfulness");
    hard_words.insert("somber", "dark and dull, oppressively serious");
    hard_words.insert("tangible", "perceptible by touch, clear and definite");
    hard_words.insert("uncanny", "strange or mysterious in a way that unsettles");
    hard_words.insert("vindictive", "having a strong desire for revenge");
    hard_words.insert("wrath", "extreme anger");
    hard_words.insert("xenophobia", "dislike of people from other countries");
    hard_words.insert("yarns", "long complicated stories");
    hard_words.insert("zeal", "great energy in pursuit of a cause");
    hard_words.insert("abyss", "a deep or seemingly bottomless pit");
    hard_words.insert("blight", "a thing that spoils or damages something");
    hard_words.insert("carnage", "the killing of a large number of people");
    hard_words.insert("dawdle", "waste time by being slow");
    hard_words.insert("eerie", "strange and frightening");
    hard_words.insert("fathom", "to understand after much thought");
    hard_words.insert("gloat", "to contemplate one's own success with smugness");
    hard_words.insert("haggard", "looking exhausted and unwell");
    hard_words.insert("inept", "having no skill at something");
    hard_words.insert("jabber", "talk rapidly and excitedly without much sense");
    hard_words.insert("knack", "an acquired skill at performing a task");
    hard_words.insert("laden", "heavily loaded or weighed down");
    hard_words.insert("mirth", "amusement expressed as laughter");
    hard_words.insert("naive", "lacking experience or judgement");
    hard_words.insert("opaque", "not able to be seen through, not clear");
    hard_words.insert("plight", "a dangerous or difficult situation");
    hard_words.insert("quirky", "having unusual and distinctive traits");
    hard_words.insert("ravage", "cause severe and extensive damage to");
    hard_words.insert("shrewd", "having sharp powers of judgement");
    hard_words.insert("turmoil", "a state of great disturbance or confusion");
    hard_words.insert("unruly", "disorderly and difficult to control");
    hard_words.insert("vivid", "producing strong clear images in the mind");
    hard_words.insert(
        "pneumonoultramicroscopicsilicolvolcanoconiosis",
        "a lung disease caused by inhaling very fine silica dust",
    );

    let easy_list: Vec<(&str, &str)> = easy_words.into_iter().collect();
    let hard_list: Vec<(&str, &str)> = hard_words.into_iter().collect();

    println!(
        "Welcome to Wordle!
------------------
How to play:
- Guess the hidden 5-letter word
- After each guess you'll see feedback for each letter:
  CorrectPosition  → right letter, right spot
  WrongPosition    → right letter, wrong spot
  NotInWord        → letter not in the word

Easy mode: 5 words, 6 guesses each, vague definitions
Hard mode: 10 words, 4 guesses each, clearer definitions

Win all easy words to unlock hard mode.
Good luck!
------------------"
    );

    for word in easy_list {
        let won = play_round(word, 6);
        if won == false {
            println!("Game over!");
            std::process::exit(0);
        }
    }
    println!("Great job! Now onto the hard words!");
    for word in hard_list {
        let won = play_round(word, 4);
        if won == false {
            println!("Game over!");
            std::process::exit(0);
        }
    }

    println!("Congratulations! You've completed the game!");
}

fn play_round(word: (&str, &str), guesses: u32) -> bool {
    println!("Definition: {}", word.1);
    for _guess in 0..guesses {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let mut results: Vec<LetterResult> = Vec::new();

        for (guess_char, secret_char) in guess.trim().chars().zip(word.0.chars()) {
            if guess_char == secret_char {
                results.push(LetterResult::CorrectPosition);
            } else if word.0.contains(guess_char) {
                results.push(LetterResult::WrongPosition);
            } else {
                results.push(LetterResult::NotInWord);
            }
        }

        println!("{:?}", results);
       
        if guess.trim() == word.0 {
            return true;
        }
    }

    return false;
}

