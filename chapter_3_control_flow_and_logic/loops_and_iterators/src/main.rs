fn main() {

    // For loop
    for i in 0..5 {
        println!("{}", i);
    }

    let v = vec![10, 20, 30];
    for x in &v { // iterate by reference
        println!("{}", x);    
    }

    // Prefer iterators over manual indexing

    // While
    let mut i = 0;
    while i < 3 {
        println!("{}", i);
        i += 1;
    }

    // Loop
    // Runs forever and yields a value
    // Useful for REPLs, retry loops, when breaking is needed and a result needs to be consolidated from the breaks
    let mut j = 0;
    let res = loop {
        if j > 10 {
            break j;
        }
        j += 1;
    };

    println!("{}", res);

    // Iterators and the Iterator trait
    // Iterator trait implements next()
    // Adapters:
    //   map, filter, enumerate, zip, take, skip, flat_map, chain
    // Consumers:
    //   collect, fold, sum, product, for_each, find, any, all
    // Iterators are lazy
    // Usually read better than mutable loops
    // Use collect::<Vec<T>>() to gather results, specify the type when ambiguous
    
    // Sum even squares of numbers
    let sum: i32 = (0..10)
        .map(|x| x * x)
        .filter(|&x| x % 2 == 0)
        .sum();
    println!("{}", sum);

    // Even numbers
    println!("{:?}", (0..100).filter(|x| x % 2 == 0).collect::<Vec<_>>());    


}


