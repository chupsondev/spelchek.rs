import sys

suggestion_dict = []

proper_dictionary = []

with open("dict.txt", "r") as f:
    proper_dictionary = f.readlines()


proper_dictionary = [x.strip() for x in proper_dictionary]

print("the" in proper_dictionary)

with open("suggestion_dict.txt", "r") as f:
    suggestion_dict = f.readlines()


n = int(input("num of elements"))

output = ""
for i in range(n):
    word = suggestion_dict[i].split()[0].strip()
    if word in proper_dictionary:
        output += suggestion_dict[i].strip() + '\n'

with open("suggestion_dict_processed.txt", "w") as f:
    f.write(output)
