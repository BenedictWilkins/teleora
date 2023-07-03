class Generator:
    def __init__(self):
        self.iters = []
        self.current_indices = []
        self.exhausted = False

    def append_iter(self, iterator):
        self.iters.append(iterator)
        self.current_indices.append(0)

    def __iter__(self):
        return self

    def __next__(self):
        if self.exhausted:
            raise StopIteration

        values = [self.iters[i][self.current_indices[i]] for i in range(len(self.iters))]

        i = len(self.iters) - 1
        while True:
            self.current_indices[i] += 1
            if self.current_indices[i] >= len(self.iters[i]):
                self.current_indices[i] = 0
                if i == 0:
                    self.exhausted = True
                    break
                i -= 1
            else:
                break

        return tuple(values)


# Example usage
iter1 = [1, 2, 3]
iter2 = [4, 5, 6]

g = Generator()
g.append_iter(iter1)
g.append_iter(iter2)

for x in g:
    print(x)
