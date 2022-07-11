use Stoichio_calc::lin_alg::Matrix;

#[test]
fn diagonalize_3x4_test(){
    let input = Matrix::of_arr(&[
        &[2, 0, -12, 0],
        &[1, 2, -6, -2],
        &[0, 1, -6, 0]
    ]);
    let expected_output = Matrix::of_arr(&[
        &[-1, 0, 0, 1],
        &[0, -1, 0, 1],
        &[0, 0, -6, 1]
    ]);
    let actual_output = input.diagonalized();
    assert!(actual_output.is_ok());
    assert_eq!(expected_output, actual_output.unwrap());
}

// TODO more tests
