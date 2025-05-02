function test(a) {
    print(a);
    test(a + 1);
}

test(1);