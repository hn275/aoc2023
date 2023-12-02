#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

std::vector<std::string> split(std::string s, char delim);
std::string trim(std::string s);

class Cube {
public:
  int blues = 0;
  int reds = 0;
  int greens = 0;

  // example input: 3 blue, 4 red
  Cube(std::string c) {
    std::vector<std::string> cubes = split(c, ',');

    for (std::string cube : cubes) {
      std::vector<std::string> d = split(cube, ' ');
      int count = std::stoi(d[0], 0, 10);
      std::string color = d[1];

      if (color == "green") {
        this->greens += count;
      } else if (color == "red") {
        this->reds += count;
      } else if (color == "blue") {
        this->blues += count;
      } else {
        throw std::invalid_argument{"invalid color, only red, green and blue"};
      }
    }
  }
  Cube() {}

  bool ok() {
    // "12 red, 13 green, 14 blue"
    bool a = this->reds <= 12;
    bool b = this->greens <= 13;
    bool c = this->blues <= 14;
    return a && b && c;
  }

  void print() {
    printf("{ red: %d, green: %d, blue: %d }\n", this->reds, this->greens,
           this->blues);
  }

  int prod() { return this->reds * this->blues * this->greens; }
};

std::vector<Cube> parse_cubes(std::string line);
Cube min_cube(std::vector<Cube>);

int main() {
  std::string line{};
  std::ifstream infile("./input.txt");
  int p1_result = 0;
  int p2_result = 0;

  while (std::getline(infile, line)) {
    std::cout << line << std::endl;
    auto s = split(line, ':');
    std::string game = split(s[0], ' ')[1];
    std::vector<Cube> cubes = parse_cubes(s[1]);
    bool game_ok = true;
    for (auto cube : cubes) {
      if (!cube.ok()) {
        game_ok = false;
        break;
      }
    }
    if (!game_ok) {
      std::cout << "Game failed: ";
      for (auto c : cubes)
        c.print();
    } else {
      p1_result += std::stoi(game);
      std::cout << "Game ok, game id: " << game << " result: " << p1_result
                << std::endl;
    }

    Cube m = min_cube(cubes);
    std::cout << "Min cube: ";
    m.print();
    p2_result += m.prod();

    std::cout << std::endl;
  }

  std::cout << "Part 1: " << p1_result << std::endl;
  std::cout << "Part 2: " << p2_result << std::endl;
  return 0;
}

Cube min_cube(std::vector<Cube> cubes) {
  Cube r = Cube();
  for (auto cube : cubes) {
    if (cube.reds > r.reds) {
      r.reds = cube.reds;
    }
    if (cube.blues > r.blues) {
      r.blues = cube.blues;
    }
    if (cube.greens > r.greens) {
      r.greens = cube.greens;
    }
  }
  return r;
}

std::vector<Cube> parse_cubes(std::string line) {
  std::vector<Cube> buf{};
  for (std::string g : split(line, ';')) {
    buf.push_back(Cube(g));
  }

  return buf;
}

std::string trim(std::string s) {
  // trim leading white space
  std::string a = std::regex_replace(s, std::regex("^\\s+"), std::string(""));
  // trim trailing white space
  return std::regex_replace(a, std::regex("\\s+$"), std::string(""));
}

std::vector<std::string> split(std::string s, char delim) {
  std::vector<std::string> buf{};
  size_t i = 0;
  std::string token = "";

  while (i < s.length()) {
    if (s.at(i) == delim) {
      buf.push_back(trim(token));
      token = "";
      i++;
      continue;
    }
    token += s.at(i);
    i++;
  }
  buf.push_back(trim(token));

  return buf;
}
