import enum

class Item:
    def __init__(self):
        self.id = 0
        self.name = ""

    def __repr__(self):
        return f"id: {self.id} name: {self.name}"
    


@enum.Enum
class MenuOptions:
    Option1 = 0
    Option2 = 1
    Option3 = 2


def fact(n):
    x = 1
    for i in range(1, n + 1):
        x += i
    return x


if __name__ == "__main__":
    nums = List.of(1, 2, 3, 4, 5)
    facts = nums.map(lambda n: fact(n))
    print(nums, end='\n')
    print(nums, end='\n')
