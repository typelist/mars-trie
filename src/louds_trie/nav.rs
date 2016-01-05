use std;
use base::*;
use super::LoudsTrie;
use super::NodeID;
use super::LoudsPos;

/*
struct History {
    node_id_: u32,
    louds_pos_: u32,
    key_pos_: u32,
    link_id_: u32,
    key_id_: u32,
}

impl History {
    fn new() -> History {
        History { node_id_: 0, louds_pos_: 0, key_pos_: 0,
                  link_id_: INVALID_LINK_ID, key_id_: INVALID_KEY_ID }
    }
    fn set_node_id(&mut self, node_id: usize) {
        assert!(node_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.node_id_ = node_id as u32;
    }
    fn set_louds_pos(&mut self, louds_pos: usize) {
        assert!(louds_pos <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.louds_pos_ = louds_pos as u32;
    }
    fn set_key_pos(&mut self, key_pos: usize) {
        assert!(key_pos <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.key_pos_ = key_pos as u32;
    }
    fn set_link_id(&mut self, link_id: usize) {
        assert!(link_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.link_id_ = link_id as u32;
    }
    fn set_key_id(&mut self, key_id: usize) {
        assert!(key_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.key_id_ = key_id as u32;
    }
    fn node_id(&self) -> usize {
        self.node_id_ as usize
    }
    fn louds_pos(&self) -> usize {
        self.louds_pos_ as usize
    }
    fn key_pos(&self) -> usize {
        self.key_pos_ as usize
    }
    fn link_id(&self) -> usize {
        self.link_id_ as usize
    }
    fn key_id(&self) -> usize {
        self.key_id_ as usize
    }
}

struct State {
    key_buf_: Vec<u8>,
    history_: Vec<History>,
    node_id_: u32,
}

impl State {
    fn new() -> State {
        State { key_buf_: Vec::new(), history_: Vec::new(), node_id_: 0,
                query_pos_: 0, history_pos_: 0, }
    }

    fn push(&mut self, node_id: NodeID, louds_pos: LoudsPos, key_pos: u32,
            link_id: u32,
    node_id_: u32,
    louds_pos_: u32,
    key_pos_: u32,
    link_id_: u32,
    key_id_: u32,
}


    fn set_node_id(&mut self, node_id: usize) {
        assert!(node_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.node_id_ = node_id as u32;
    }
    fn get_node_id(&self) -> usize {
        self.node_id_ as usize
    }
    fn reset(&mut self) {
        *self = State::new();
    }
}
*/

struct History<'a> {
    trie_: &'a LoudsTrie
    node_id_: NodeID,
    louds_pos_: LoudsPos,
    link_id_: LinkID,
    key_pos_: u32,
    //key_id_: u32,
}

impl<'a> History<'a> {
    fn new(trie: &'a LoudsTrie, node_id: NodeID, louds_pos: LoudsPos,
           link_id: LinkID, key_pos: u32) -> History<'a> {
        History { trie_: trie, node_id_: node_id, louds_pos_: louds_pos,
                  link_id_: link_id, key_pos_: key_pos }
    }
}

struct State<'a> {
    history_: Vec<History>,
    key_buf_: Vec<u8>,
}

impl<'a> State<'a> {
    fn new() -> State<'a> {
        State { history_: Vec::new(), key_buf_: Vec::new() }
    }

    fn push<'b>(&'mut self, key: &'b[u8], trie: &'a LoudsTrie, node_id: NodeID,
                louds_pos: LoudsPos, link_id: LinkID, key_pos: u32) {

        self.history_.push_back(
        

    }

    fn pop(&'mut self) -> History<'a>
}

pub struct Nav<'a> {
    state_: State,
    trie_: &'a LoudsTrie,
}

//struct LoudsPos(u32);
//struct NodeID(u32);

impl Nav<'a> {
    pub fn new<'a>(trie: &'a LoudsTrie) -> Nav<'a> {
        Nav { state_: State::new(), trie_: trie }
    }

    //pub fn has_child(&self) -> bool {
    //fn child_pos(&self) -> Option<(NodeID, LoudsPos)> {
    pub fn go_to_child(&mut self) -> bool {
        // For lookups, marisa does caching based on the input character.
        // We can't do that here. May want to remove or rethink the cache
        // implementation in light of this.

        //let louds = &self.trie_.louds_;
        //let state = &mut self.state_;
        //let link_flags = &self.trie_.link_flags_;

        if let Some((node_id, louds_pos))
        = self.trie_.child_pos(self.state_.get_node_id()) {
    
            let mut link_id = INVALID_LINK_ID;
            do {
                if link_flags[state.node_id()] {
                    //link_id = update_link_id(link_id, state.node_id());
    
                    //const std::size_t prev_query_pos = state.query_pos();
                    //if (match(agent, get_link(state.node_id(), link_id))) {
                    //  return true;
                    //} else if (state.query_pos() != prev_query_pos) {
                    //  return false;
                    //}
                } else {
                    // Character for node 
                    bases_[state.node_id()]

                    state.set_query_pos(state.query_pos() + 1);
                    return true;
                }
                state.set_node_id(state.node_id() + 1);
                ++louds_pos;
            } while (louds_[louds_pos]);
            false

        } else {
            false
        }
    }
    pub fn has_sibling(&self) -> bool {
        panic!("not implemented")
    }
    pub fn go_to_sibling(&mut self) -> bool {
        panic!("not implemented")
    }
    pub fn has_parent(&self) -> bool {
        panic!("not implemented")
    }
    pub fn go_to_parent(&self) -> bool {
    }
}

struct State<'a> {
    history_: Vec<History>,
    key_buf_: Vec<u8>,
}

impl<'a> State<'a> {
    fn new() -> State<'a> {
        State { history_: Vec::new(), key_buf_: Vec::new() }
    }

    fn push<'b>(&'mut self, key: &'b[u8], trie: &'a LoudsTrie, node_id: NodeID,
                louds_pos: LoudsPos, link_id: LinkID, key_pos: u32) {

        self.history_.push_back(
        

    }

    fn pop(&'mut self) -> History<'a>
}

pub struct Nav<'a> {
    state_: State,
    trie_: &'a LoudsTrie,
}

//struct LoudsPos(u32);
//struct NodeID(u32);

impl Nav<'a> {
    pub fn new<'a>(trie: &'a LoudsTrie) -> Nav<'a> {
        Nav { state_: State::new(), trie_: trie }
    }

    //pub fn has_child(&self) -> bool {
    //fn child_pos(&self) -> Option<(NodeID, LoudsPos)> {
    pub fn go_to_child(&mut self) -> bool {
        // For lookups, marisa does caching based on the input character.
        // We can't do that here. May want to remove or rethink the cache
        // implementation in light of this.

        //let louds = &self.trie_.louds_;
        //let state = &mut self.state_;
        //let link_flags = &self.trie_.link_flags_;

        if let Some((node_id, louds_pos))
        = self.trie_.child_pos(self.state_.get_node_id()) {
    
            let mut link_id = INVALID_LINK_ID;
            do {
                if link_flags[state.node_id()] {
                    //link_id = update_link_id(link_id, state.node_id());
    
                    //const std::size_t prev_query_pos = state.query_pos();
                    //if (match(agent, get_link(state.node_id(), link_id))) {
                    //  return true;
                    //} else if (state.query_pos() != prev_query_pos) {
                    //  return false;
                    //}
                } else {
                    // Character for node 
                    bases_[state.node_id()]

                    state.set_query_pos(state.query_pos() + 1);
                    return true;
                }
                state.set_node_id(state.node_id() + 1);
                ++louds_pos;
            } while (louds_[louds_pos]);
            false

        } else {
            false
        }
    }
    pub fn has_sibling(&self) -> bool {
        panic!("not implemented")
    }
    pub fn go_to_sibling(&mut self) -> bool {
        panic!("not implemented")
    }
    pub fn has_parent(&self) -> bool {
        panic!("not implemented")
    }
    pub fn go_to_parent(&self) -> bool {
        panic!("not implemented")
    }
    pub fn is_terminal(&self) -> bool {
        panic!("not implemented")
    }
    pub fn get_string(&self) -> &str {
        panic!("not implemented")
    }
    pub fn is_end(&self) -> bool {
        panic!("not implemented")
    }

}

/*

bool LoudsTrie::lookup(Agent &agent) const {
  MARISA_DEBUG_IF(!agent.has_state(), MARISA_STATE_ERROR);

  State &state = agent.state();
  state.lookup_init();
  while (state.query_pos() < agent.query().length()) {
    if (!find_child(agent)) {
      return false;
    }
  }
  if (!terminal_flags_[state.node_id()]) {
    return false;
  }
  agent.set_key(agent.query().ptr(), agent.query().length());
  agent.set_key(terminal_flags_.rank1(state.node_id()));
  return true;
}

bool LoudsTrie::find_child(Agent &agent) const {
  MARISA_DEBUG_IF(agent.state().query_pos() >= agent.query().length(),
      MARISA_BOUND_ERROR);

  State &state = agent.state();
  const std::size_t cache_id = get_cache_id(state.node_id(),
      agent.query()[state.query_pos()]);
  if (state.node_id() == cache_[cache_id].parent()) {
    if (cache_[cache_id].extra() != MARISA_INVALID_EXTRA) {
      if (!match(agent, cache_[cache_id].link())) {
        return false;
      }
    } else {
      state.set_query_pos(state.query_pos() + 1);
    }
    state.set_node_id(cache_[cache_id].child());
    return true;
  }

  std::size_t louds_pos = louds_.select0(state.node_id()) + 1;
  if (!louds_[louds_pos]) {
    return false;
  }
  state.set_node_id(louds_pos - state.node_id() - 1);
  std::size_t link_id = MARISA_INVALID_LINK_ID;
  do {
    if (link_flags_[state.node_id()]) {
      link_id = update_link_id(link_id, state.node_id());
      const std::size_t prev_query_pos = state.query_pos();
      if (match(agent, get_link(state.node_id(), link_id))) {
        return true;
      } else if (state.query_pos() != prev_query_pos) {
        return false;
      }
    } else if (bases_[state.node_id()] ==
        (UInt8)agent.query()[state.query_pos()]) {
      state.set_query_pos(state.query_pos() + 1);
      return true;
    }
    state.set_node_id(state.node_id() + 1);
    ++louds_pos;
  } while (louds_[louds_pos]);
  return false;
}

std::size_t LoudsTrie::get_cache_id(std::size_t node_id, char label) const {
  return (node_id ^ (node_id << 5) ^ (UInt8)label) & cache_mask_;
}

std::size_t LoudsTrie::get_cache_id(std::size_t node_id) const {
  return node_id & cache_mask_;
}

bool LoudsTrie::match(Agent &agent, std::size_t link) const {
  if (next_trie_.get() != NULL) {
    return next_trie_->match_(agent, link);
  } else {
    return tail_.match(agent, link);
  }
}

bool LoudsTrie::match_(Agent &agent, std::size_t node_id) const {
  MARISA_DEBUG_IF(agent.state().query_pos() >= agent.query().length(),
      MARISA_BOUND_ERROR);
  MARISA_DEBUG_IF(node_id == 0, MARISA_RANGE_ERROR);

  State &state = agent.state();
  for ( ; ; ) {
    const std::size_t cache_id = get_cache_id(node_id);
    if (node_id == cache_[cache_id].child()) {
      if (cache_[cache_id].extra() != MARISA_INVALID_EXTRA) {
        if (!match(agent, cache_[cache_id].link())) {
          return false;
        }
      } else if (cache_[cache_id].label() ==
          agent.query()[state.query_pos()]) {
        state.set_query_pos(state.query_pos() + 1);
      } else {
        return false;
      }

      node_id = cache_[cache_id].parent();
      if (node_id == 0) {
        return true;
      } else if (state.query_pos() >= agent.query().length()) {
        return false;
      }
      continue;
    }

    if (link_flags_[node_id]) {
      if (next_trie_.get() != NULL) {
        if (!match(agent, get_link(node_id))) {
          return false;
        }
      } else if (!tail_.match(agent, get_link(node_id))) {
        return false;
      }
    } else if (bases_[node_id] == (UInt8)agent.query()[state.query_pos()]) {
      state.set_query_pos(state.query_pos() + 1);
    } else {
      return false;
    }

    if (node_id <= num_l1_nodes_) {
      return true;
    } else if (state.query_pos() >= agent.query().length()) {
      return false;
    }
    node_id = louds_.select1(node_id) - node_id - 1;
  }
}

bool Tail::match(Agent &agent, std::size_t offset) const {
  MARISA_DEBUG_IF(buf_.empty(), MARISA_STATE_ERROR);
  MARISA_DEBUG_IF(agent.state().query_pos() >= agent.query().length(),
      MARISA_BOUND_ERROR);

  State &state = agent.state();
  if (end_flags_.empty()) {
    const char * const ptr = &buf_[offset] - state.query_pos();
    do {
      if (ptr[state.query_pos()] != agent.query()[state.query_pos()]) {
        return false;
      }
      state.set_query_pos(state.query_pos() + 1);
      if (ptr[state.query_pos()] == '\0') {
        return true;
      }
    } while (state.query_pos() < agent.query().length());
    return false;
  } else {
    do {
      if (buf_[offset] != agent.query()[state.query_pos()]) {
        return false;
      }
      state.set_query_pos(state.query_pos() + 1);
      if (end_flags_[offset++]) {
        return true;
      }
    } while (state.query_pos() < agent.query().length());
    return false;
  }
}

*/
