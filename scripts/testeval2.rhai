fn rand() {
  4
}

let adjectives = ["Compassionate", "Intelligent", "Creative", "Ambitious", "Charismatic", "Confident", "Courageous", "Diligent", "Enthusiastic", "Friendly", "Generous", "Humorous", "Imaginative", "Kind", "Loyal", "Motivated", "Optimistic", "Perceptive", "Resilient", "Thoughtful"];
let randomIndexes = [];

print(adjectives.len());

for i in 0..3 {
    let randomIndex = rand();
    randomIndexes.push(randomIndex);
}

let selectedAdjectives = [];

for index in randomIndexes {
  print(index)

}
