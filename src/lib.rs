fn multipleofthreeandfive(n : i64) -> i64 {
    let multiplesfive = n / 5;
    let multiplesthree = n / 3;
    let multiplesfifteen = n / 15;
    println!("{} {} {}",multiplesfive,multiplesthree,multiplesfifteen);
    let sumfive = multiplesfive * multiplesfive * 5;
    let sumthree = multiplesthree * multiplesthree * 3;
    let sumfifteen = multiplesfifteen * multiplesfifteen * 15;
    println!("{} {} {}",sumfive,sumthree,sumfifteen);
    sumfive + sumthree - sumfifteen
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multipleofthreeandfivetest() {
        assert_eq!(multipleofthreeandfive(0),0);
        assert_eq!(multipleofthreeandfive(1),0);
        assert_eq!(multipleofthreeandfive(3),3);
        assert_eq!(multipleofthreeandfive(5),8);
        assert_eq!(multipleofthreeandfive(10),23);
        //assert_eq!(multipleofthreeandfive(15),15);
        assert_eq!(multipleofthreeandfive(1000),266333)
    }
}