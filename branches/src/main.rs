fn main() {

    let num = 5;
    if num>3 {
        println!("Num was larger than 3");
    }

    
    // runs forever
    // useful to retry an operation known to fail/take time (threading)
    // can return a value with break <val>
    let result = loop {
        println!("Running in a loop now");
        break 5;
    };
    println!("Results was {result}");


    // loops can be labelled, allows to break outermost loops

    
    let mut cnt1 = 0;
    'label1: loop {
        let mut cnt2 = 0;
        'label2: loop{
            if cnt2 >= 5 {
                break 'label1;  // breaks external, stops everything
            }
            println!("cnt2: {cnt2}");
            cnt2 = cnt2 + 1;
        }
        if cnt1 >= 5 {
            break;
        }
        println!("cnt1: {cnt1}");
        cnt1 = cnt1 + 1;

    }

    let strings = ["aa", "bb", "cc"];

    for s in strings {
        println!("String: {s}");
    }

}
