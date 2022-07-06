pub fn are_vecs_equal<T: PartialEq>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    let zipped = vec1.iter().zip(vec2.iter());
    let filtered = zipped.filter(|&(from_first, from_second)| from_first == from_second);
    let t = filtered.count();

    t == vec1.len()
}

#[cfg(test)]
mod test_are_vecs_equal {
    use super::*;

    #[test]
    fn test_equal() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![1, 2, 3];

        assert!(are_vecs_equal(&vec1, &vec2))
    }

    #[test]
    fn test_equal_struct() {
        #[derive(PartialEq, Eq)]
        struct SomeStruct {
            field1: String,
            field2: u32,
        }

        let vec1 = vec![
            SomeStruct {
                field1: "hello".to_string(),
                field2: 123,
            },
            SomeStruct {
                field1: "qwerty".to_string(),
                field2: 321,
            },
        ];

        let vec2 = vec![
            SomeStruct {
                field1: "hello".to_string(),
                field2: 123,
            },
            SomeStruct {
                field1: "qwerty".to_string(),
                field2: 321,
            },
        ];

        assert!(are_vecs_equal(&vec1, &vec2))
    }

    #[test]
    fn test_not_equal_struct() {
        #[derive(PartialEq, Eq)]
        struct SomeStruct {
            field1: String,
            field2: u32,
        }

        let vec1 = vec![
            SomeStruct {
                field1: "hello".to_string(),
                field2: 123,
            },
            SomeStruct {
                field1: "qwerty".to_string(),
                field2: 321,
            },
        ];

        let vec2 = vec![SomeStruct {
            field1: "asd".to_string(),
            field2: 152,
        }];

        assert!(!are_vecs_equal(&vec1, &vec2))
    }
}
