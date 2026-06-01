#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
name: String,
contents: Vec<File>
}

impl Folder {
    fn new(name: String) -> Self {
        Self { 
            name, 
            contents: vec![]
         }
    }

    fn create_file(&mut self, name: String){
       let file =  File { name };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index:usize) -> File {
        self.contents.remove(index)
    }

    fn get_file( &self, index:usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main(){
    let mut my_folder = Folder::new(String::from("Docs"));

    my_folder.create_file(String::from("rust.rs"));
    my_folder.create_file(String::from("python.py"));
    println!("{:#?}", my_folder);

    my_folder.delete_file(1);
    println!("{:#?}", my_folder);

    match my_folder.get_file(0) {
        Some(s) => println!("{:?}", s),
        None => println!("There was no file"),
    };

}

  
