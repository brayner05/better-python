def factorial(n):
	if n == 0:
	return 1
	return n * factorial(n - 1)
if __name__ == "__main__":
	print(factorial(1))
	print(factorial(2))
	print(factorial(3))
	print(factorial(4))
	print(factorial(5))
