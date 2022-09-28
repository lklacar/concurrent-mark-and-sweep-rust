
int main() {
    long i = 0;
    long sum = 0;
    while (i < 1000000000) {
        sum = sum + i;
        i = i + 1;
    }
    return sum;
}