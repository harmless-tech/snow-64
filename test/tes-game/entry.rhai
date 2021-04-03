import "entity" as entity;

let keep = 1;

start(keep);

fn start(num) {
    display("Hello");

    let e = entity.Entity;
    print(num);
    print(e.x);
}
