use std::cmp::Reverse;

// zbior reszt modulo n ktore sa w pierwsze w stosunku do n jest oznaczona jako Zn*
// kazdy element Zn* ma odwrotnosc multiplikatywną
// 26 === 11 (mod 5) bo reszta z dzielenia 26 przez 5 == 1 i reszta z dzielenia 11 przez 5 = 1
// M to iloczyn m1, ..., mr
// mnozenie i sumowanie modulo M
// potegowanie modulo to podniesienie do kwadratu
// gdzie a^e, zapisac e w postaci binarnej

use std::process::abort;

fn NWD(j: u32, k: u32) -> u32 {
    if j == 0 {
        return k
    }
    let r = k % j;
    return NWD(r, j);
}

fn modulo_euclid(j: i32, k: i32) -> i32 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}

fn rozNWD(j: i32, k: i32) -> (i32, i32, i32) {
    if j == 0 {
        return (k, 0, 1)
    }
    // let r = k % j;
    let r = modulo_euclid(k, j);
    // let r = k % j;
    let (d, x_prim, y_prim) = rozNWD(r, j);
    let x = y_prim - (k/j) * x_prim;
    let y = x_prim;
    return (d, x, y);

    // (k, 1, 0)
}

// fn rozNWD_wrapper(j: i32, k: i32) -> i32{
//     let (d, x, y) = rozNWD(j, k);
//     if x < 0 {x + k} else {x}
// }

// x: i32, n: i32
fn odwrotnosc_multiplikatywna(j: i32, k: i32) -> i32 {
    // a to jest 17 czyli j
    // n = 101 czyli k
    let (_d, x, _y) = rozNWD(j, k);
    // println!("d = {d}, x = {x}, y = {y}");
    return modulo_euclid(x, k);
}




fn main() {
    println!("Hello, world!");

    zadanieA();
    zadanieB();
    zadanieC();
    zadanieD();
    zadanieE();
    zadanieF();


    println!("Hello, world!");
}

fn zadanieF(){
    println!("zadanie F");

    let p = 12987461;
    let q = 1291;
    let g = 3606738;

    let a = 357;
    let b = 199;

    let d = 1;

    let a_binary = create_hex_binary(a);
    // let a_too =  binary_hex_to_value(a_binary.clone());
    println!("a_binary = {:?}", a_binary);
    // println!("a_too: {:?}", a_too);
    let b_binary = create_hex_binary(b);
    println!("b_binary = {:?}", b_binary);



    println!();
}

fn potegowanie(a: u32, e: u32, n: u32) -> u32 {
    let e_binary = create_hex_binary(e);
    let mut d = 1;
    let mut i = e_binary.len();
    while(i > 0){
        d = modulo_euclid(d*d, n as i32);
        if e_binary[i] == 1{
            d = modulo_euclid(d * a as i32, n as i32)
        }

        i -= 1;
    }
    return d as u32;


    // 0
    // for(let i = e_binary.len(); i > 0; i -=1){
    //
    // }
}

fn create_hex_binary(hex_value: u32) -> Vec<u32>{
    fn create_hex_binary_rec(hex_value: u32, counter: i32, acc: Vec<u32>) -> Vec<u32>{
        if counter < 0 {
            return acc;
        }
        let two_value = (2 as u32).pow(counter as u32) as u32;
        let new_counter = counter - 1;
        if hex_value >= two_value{
            let new_acc = functional_push_right(acc, 1);
            create_hex_binary_rec(hex_value - two_value, new_counter, new_acc)
        }else{
            let new_acc = functional_push_right(acc, 0);
            create_hex_binary_rec(hex_value, new_counter, new_acc)
        }
    }
    create_hex_binary_rec(hex_value, 10, vec![])
}

fn binary_hex_to_value(binary_hex: Vec<u32>) -> u32{
    return reverse((0..binary_hex.len()).map(|x| x as u32).collect::<Vec<u32>>()).into_iter().zip(binary_hex.into_iter()).map(|(i, ei)| {
        (2 as u32).pow(i) * ei
    }).reduce(|acc, x| acc + x).unwrap();

    0
    // 8 * binary_hex.get(0).unwrap() + 4 * binary_hex.get(1).unwrap() + 2 * binary_hex.get(2).unwrap() + binary_hex.get(3).unwrap()
}

fn reverse(vec: Vec<u32>) -> Vec<u32>{
    let mut vec_clone = vec.clone();
    vec_clone.sort_by_key(|&num| (true, Reverse(num)));
    return vec_clone;
}

fn functional_push_right(vec: Vec<u32>, value: u32) -> Vec<u32>{

    vec.into_iter().chain([value].into_iter()).collect()
}



fn zadanieE(){
    println!("zadanie E");
    let odwrotnosc_multiplikatywna_13_mod_99 = odwrotnosc_multiplikatywna(13, 99);
    let odwrotnosc_multiplikatywna_15_mod_101 = odwrotnosc_multiplikatywna(15, 101);
    println!("odwrotnosc_multiplikatywna_13_mod_99: {odwrotnosc_multiplikatywna_13_mod_99}");
    println!("odwrotnosc_multiplikatywna_15_mod_101: {odwrotnosc_multiplikatywna_15_mod_101}");

    let a1 = modulo_euclid(4 * odwrotnosc_multiplikatywna_13_mod_99, 99);
    let a2 = modulo_euclid(56 * odwrotnosc_multiplikatywna_15_mod_101, 101);
    let a_array = vec![a1, a2];
    let m_array = vec![99, 101];
    let M = m_array.clone().into_iter().reduce(|acc, x| acc * x).unwrap();
    println!("M = {}", M);

    let sum_parts: Vec<i32> = m_array.clone().into_iter().zip(a_array.clone()).map(|(m_i, a_i)| {
        let M_i: i32 = (M as f32 / m_i as f32) as i32;
        let y_i = odwrotnosc_multiplikatywna(M_i, m_i);
        let res = modulo_euclid(a_i * M_i * y_i, M);
        return res;
    }).collect();
    let x = modulo_euclid(sum_parts.clone().into_iter().reduce(|acc, x| acc + x).unwrap(), M);
    println!("x: {x}");
    println!();
}

fn zadanieD(){
    println!("zadanie D");

    let a_array = vec![12, 9, 23];
    let m_array = vec![25, 26, 27];
    let M = m_array.clone().into_iter().reduce(|acc, x| acc * x).unwrap();
    println!("M = {}", M);
    // let mut y_array = vec![];

    let sum_parts: Vec<i32> = m_array.clone().into_iter().zip(a_array.clone()).map(|(m_i, a_i)| {
        let M_i: i32 = (M as f32 / m_i as f32) as i32;
        let y_i = odwrotnosc_multiplikatywna(M_i, m_i);
        let res = modulo_euclid(a_i * M_i * y_i, M);
        return res;
    }).collect();
    let x = modulo_euclid(sum_parts.clone().into_iter().reduce(|acc, x| acc + x).unwrap(), M);
    println!("x: {x}");
    println!();
}



fn zadanieC(){
    println!("zadanie C");

    let m_array = vec![3, 5, 7];
    let M = m_array.clone().into_iter().reduce(|acc, x| acc * x).unwrap();
    println!("M = {}", M);
    let a_array = vec![2, 2, 3];

    // modulo_euclid
    let mut y_array = vec![];

    let sum_parts: Vec<i32> = m_array.clone().into_iter().zip(a_array.clone()).map(|(m_i, a_i)| {
        let M_i: i32 = (M as f32 / m_i as f32) as i32;
        // println!("M_i = {}", M_i);
        let y_i = odwrotnosc_multiplikatywna(M_i, m_i);
        y_array.push(y_i);
        // println!("y_i = {}", y_i);
        let res = modulo_euclid(a_i * M_i * y_i, M);
        // let res = modulo_euclid(a_i * M_i, M) * modulo_euclid(y_i, M);
        // println!();
        return res;
    }).collect();
    let x = modulo_euclid(sum_parts.clone().into_iter().reduce(|acc, x| acc + x).unwrap(), M);
    let Ksi_parts: Vec<i32> = sum_parts.clone().into_iter().zip(y_array.into_iter()).map(|(sum_part, y_i)| {
        sum_part * y_i
    }).collect();
    //

    println!("x: {x}");
    println!("funkcja X^-1(x) = (a1 * {}, a2 * {}, a3 * {}) mod {M}",
             Ksi_parts[0],
             Ksi_parts[1],
             Ksi_parts[2]);
    println!();
}


fn zadanieB(){
    println!("zadanie B");
    let odwrotnosc_multiplikatywna_17_mod_101 = odwrotnosc_multiplikatywna(17, 101);
    let odwrotnosc_multiplikatywna_357_mod_1234 = odwrotnosc_multiplikatywna(357, 1234);
    let odwrotnosc_multiplikatywna_3125_mod_9987 = odwrotnosc_multiplikatywna(3125, 9987);

    println!("odwrotnosc_multiplikatywna_17_mod_101: {:?}", odwrotnosc_multiplikatywna_17_mod_101);
    println!("odwrotnosc_multiplikatywna_357_mod_1234: {:?}", odwrotnosc_multiplikatywna_357_mod_1234);
    println!("odwrotnosc_multiplikatywna_3125_mod_9987: {:?}", odwrotnosc_multiplikatywna_3125_mod_9987);
    println!();
}


fn zadanieA(){
    println!("zadanie A");
    let nwd_57_93 = NWD(57, 93);
    println!("nwd_57_93: {:?}", nwd_57_93);
    println!();
}

