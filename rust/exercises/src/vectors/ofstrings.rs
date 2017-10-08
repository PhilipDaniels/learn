fn Make() -> Vec<String> {
    vec![
        "aa".to_owned(),
        "dd".to_owned(),
        "cc".to_owned(),
        "Hello".to_owned(),
        "World".to_owned(),
    ]
}

pub fn run() {
    println!("********* vec<String> examples *********");

    let mut v = Make();
    println!("v: {:?}", &v);

    v.sort();
    println!("v after sorting: {:?}", &v);

    let r = v.join(",");
    println!("r after join(): {:?}", &r);

    {
        let r = &mut v[0];
        r.push_str(" Phil!");
    }

    println!("v after changing the first String: {:?}", &v);

    // Convert to another form.
    let v2 : Vec<&str> = v.iter().map(|elem| elem.as_str()).collect();


    let mut v = Make();
    for s in v.iter_mut() {
        s.insert_str(0, "PREFIX ");
    }
    println!("v after prefixing: {:?}", &v);

    // Title algorithm from qork.
    let v = vec![
        "alpha".to_owned(),
        "new".to_owned(),
        "beta".to_owned(),
        "new 1".to_owned(),
        "new whatever".to_owned(),
        "new 3".to_owned(),
    ];

    let proposed = "new";
    let mut titles = v.iter()
        .filter(|s| s.starts_with(&proposed))
        .map(|s| s.as_str())
        .collect::<Vec<_>>();

    let s = get_unique_title(&proposed, &titles);
    println!("s = {:?}", &s);


    let v2 = v.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let s = get_unique_title(&proposed, &v2);
    println!("s = {:?}", &s);
}

fn get_unique_title(proposed: &str, existing_titles: &[&str]) -> String {
    if !existing_titles.contains(&proposed) {
        println!("The vector {:?} does not contain {:?}", &existing_titles, &proposed);
        return proposed.to_string()
    }

    let mut i = 1;
    loop {
        let proposed = format!("{} {}", &proposed, i);
        if !existing_titles.contains(&proposed.as_str()) {
            return proposed;
        }

        i += 1;
    }
}

// How to write a function that takes an iterator and call it with an iterator,
// a slice, or a vector?
fn get_unique_title_for_iterator<'a, I>(proposed: &str, existing_titles: &mut I) -> String
    where I: Iterator<Item=&'a str> {

    if existing_titles.find(|elem| elem == &proposed).is_none() {
        return proposed.to_string()
    }

    let mut i = 1;
    loop {
        let proposed = format!("{} {}", &proposed, i);
        if existing_titles.find(|elem| elem == &proposed).is_none() {
            return proposed;
        }

        i += 1;
    }
}


/*
   fn all_buffers(&self) -> Vec<Ref<Buffer>> {
        let x : Vec<_> = self.buffers.values().map(|rcb| rcb.borrow()).collect();
        x
    }

    /// Title algorithm. We need the ability to uniqueify buffer names. Once a suffix number is
    /// assigned it is never changed. They can be reused, or even not used (we only allocate them
    /// for our convenience, and they do not need to go up monotonically like buffer ids do).
    /// For new buffers not backed by a file "new", then "new 1 " etc.
    /// For buffers backed by a file, the leaf filename, then '1' etc.
    fn get_unique_title(&self, proposed: &str) -> String {
        //let titles : Vec<_> = self.buffers.values().map(|refcell| refcell.borrow().title.clone()).collect();
        let titles : Vec<_> = self.all_buffers().iter().map(|x| x.title.clone()).collect();

        inner_get_unique_title(proposed, &titles)
    }
}

fn inner_get_unique_title(proposed: &str, current_titles: &[String]) -> String {
    let namer = |n: i32| -> String {
        if n == 0 {
            return String::from(proposed)
        } else {
            return String::from(proposed) + " " + &n.to_string()
        }
    };

    let mut i = 0;
    loop {
        let p = namer(i);
        if !current_titles.contains(&p) {
            return p
        } else {
            i += 1
        }
    }
}
*/