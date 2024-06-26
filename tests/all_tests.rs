mod infra;

// Your tests go here!
success_tests! {
    {
        name: make_vec_succ,
        file: "make_vec.snek",
        input: "5",
        expected: "[0, 0, 0, 0, 0]",
    },
    {
        name: vec_succ,
        file: "vec.snek",
        expected: "[0, 1, 2, 3]",
    },
    {
        name: vec_get_succ,
        file: "vec_get.snek",
        input: "3",
        expected: "3",
    },
    {
        name: linked_list_manipulations,
        file: "linked_list_manipulations.snek",
        expected: "1\n2\n3\n4\n5\n5\n4\n3\n2\n1\nnil"
    },
    {
        name: student_01_1,
        file: "student_01.snek",
        input: "10",
        heap_size:36,
        expected:"[[[[[[...], 7], 4], 3], 6], 2]\n[[[[[[[...], 7], 4], 3], 6], 2], 888, 888, 888, 888, 888, 888, 888, 888, 888]"
    }
}

runtime_error_tests! {
    {
        name: make_vec_oom,
        file: "make_vec.snek",
        input: "5",
        heap_size: 5,
        expected: "out of memory",
    },
    {
        name: student_01_2,
        file: "student_01.snek",
        input: "11",
        heap_size:36,
        expected:"out of memory"
    },

    {
        name: vec_get_oob,
        file: "vec_get.snek",
        input: "5",
        expected: "",
    }
}

static_error_tests! {}
