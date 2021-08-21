import random

AMOUNT = 500
print("test:")

for i in range(AMOUNT):
    sleep_time = random.triangular(0.2, 1.5)

    print(f"  test {i:0>10}:")
    print(f"    test: |-")
    # print(f"      echo 'test {i:0>10}'")
    print(f"      sleep {sleep_time}")
