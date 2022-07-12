use Stoichio_calc::lin_alg::Matrix;

#[test]
fn diagonalize_3x4_test(){
    let input = Matrix::of_arr(&[
        &[2, 0, -12, 0],
        &[1, 2, -6, -2],
        &[0, 1, -6, 0]
    ]);
    let expected_output = Matrix::of_arr(&[
        &[1, 0, 0, -1],
        &[0, 1, 0, -1],
        &[0, 0, 6, -1]
    ]);
    let actual_output = input.diagonalized();
    assert!(actual_output.is_ok());
    assert_eq!(expected_output, actual_output.unwrap());
}

#[test]
fn diagonalize_2x4_test(){
    let input = Matrix::of_arr(&[
        &[1, 0, -1, -1],
        &[3, 1, -4, 0]
    ]);
    let expected_output = Matrix::of_arr(&[
        &[1, 0, -1, -1],
        &[0, 1, -1, 3]
    ]);
    let actual_output = input.diagonalized();
    assert!(actual_output.is_ok());
    assert_eq!(expected_output, actual_output.unwrap());
}

#[test]
fn diagonalize_3x3_test(){
    let input = Matrix::of_arr(&[
        &[4, 0, -1],
        &[10, 1, -4],
        &[0, 2, -3]
    ]);
    let expected_output = Matrix::of_arr(&[
        &[4, 0, -1],
        &[0, 2, -3],
        &[0, 0, 0]
    ]);
    let actual_output = input.diagonalized();
    assert!(actual_output.is_ok());
    assert_eq!(expected_output, actual_output.unwrap());
}

#[test]
fn diagonalize_5x4_test(){
    let input = Matrix::of_arr(&[
        &[2, 0, -1, 0],
        &[3, 0, 0, -1],
        &[12, 2, -3, -4],
        &[0, 1, 0, -1],
        &[0, 2, -3, 0]
    ]);
    let expected_output = Matrix::of_arr(&[
        &[3, 0, 0, -1],
        &[0, 1, 0, -1],
        &[0, 0, 3, -2],
        &[0, 0, 0, 0],
        &[0, 0, 0, 0]
    ]);
    let actual_output = input.diagonalized();
    assert!(actual_output.is_ok());
    assert_eq!(expected_output, actual_output.unwrap());
}
