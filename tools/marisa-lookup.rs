// Copyright (c) 2010-2013, Susumu Yata
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// - Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
// - Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

#include <iostream>
#include <string>

#include <marisa.h>

#include "cmdopt.h"

namespace {

bool mmap_flag = true;

void print_help(const char *cmd) {
  std::cerr << "Usage: " << cmd << " [OPTION]... DIC\n\n"
      "Options:\n"
      "  -m, --mmap-dictionary  use memory-mapped I/O to load a dictionary"
      " (default)\n"
      "  -r, --read-dictionary  read an entire dictionary into memory\n"
      "  -h, --help             print this help\n"
      << std::endl;
}

int lookup(const char * const *args, std::size_t num_args) {
  if (num_args == 0) {
    std::cerr << "error: dictionary is not specified" << std::endl;
    return 10;
  } else if (num_args > 1) {
    std::cerr << "error: more than one dictionaries are specified"
        << std::endl;
    return 11;
  }

  marisa::Trie trie;
  if (mmap_flag) {
    try {
      trie.mmap(args[0]);
    } catch (const marisa::Exception &ex) {
      std::cerr << ex.what() << ": failed to mmap a dictionary file: "
          << args[0] << std::endl;
      return 20;
    }
  } else {
    try {
      trie.load(args[0]);
    } catch (const marisa::Exception &ex) {
      std::cerr << ex.what() << ": failed to load a dictionary file: "
          << args[0] << std::endl;
      return 21;
    }
  }

  marisa::Agent agent;
  std::string str;
  while (std::getline(std::cin, str)) {
    try {
      agent.set_query(str.c_str(), str.length());
      if (trie.lookup(agent)) {
        std::cout << agent.key().id() << '\t' << str << '\n';
      } else {
        std::cout << "-1\t" << str << '\n';
      }
    } catch (const marisa::Exception &ex) {
      std::cerr << ex.what() << ": lookup() failed: " << str << std::endl;
      return 30;
    }

    if (!std::cout) {
      std::cerr << "error: failed to write results to standard output"
          << std::endl;
      return 30;
    }
  }

  return 0;
}

}  // namespace

int main(int argc, char *argv[]) {
  std::ios::sync_with_stdio(false);

  ::cmdopt_option long_options[] = {
    { "mmap-dictionary", 0, NULL, 'm' },
    { "read-dictionary", 0, NULL, 'r' },
    { "help", 0, NULL, 'h' },
    { NULL, 0, NULL, 0 }
  };
  ::cmdopt_t cmdopt;
  ::cmdopt_init(&cmdopt, argc, argv, "mrh", long_options);
  int label;
  while ((label = ::cmdopt_get(&cmdopt)) != -1) {
    switch (label) {
      case 'm': {
        mmap_flag = true;
        break;
      }
      case 'r': {
        mmap_flag = false;
        break;
      }
      case 'h': {
        print_help(argv[0]);
        return 0;
      }
      default: {
        return 1;
      }
    }
  }
  return lookup(cmdopt.argv + cmdopt.optind, cmdopt.argc - cmdopt.optind);
}
