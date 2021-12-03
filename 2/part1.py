import data

x = 0
y = 0

for cmd in data.input_text.split("\n"):
    s = cmd.split(" ")
    direction = s[0]
    value = int(s[1])

    if direction[0] == "d":
        y += value
    if direction[0] == "u":
        y -= value
    if direction[0] == "f":
        x += value

print(f"{x=}")
print(f"{y=}")
print(f"res = {x*y}")

