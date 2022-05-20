var searchIndex = JSON.parse('{\
"lab1_grupp_9":{"doc":"","t":[0,0,0,0,0,5,0,3,4,13,13,13,13,13,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,12,11,11,3,11,11,12,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,13,13,13,3,13,4,3,13,11,11,11,11,11,11,11,11,11,12,11,5,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,5,11,11,11,5,11,11,12,11,11,5,5,5,11,11,11,11],"n":["card","deck","hand","input","logic","main","utils","Card","CardSuit","Clove","Diamond","Heart","Joker","Spade","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","convert_bytes_to_integer","eq","fmt","fmt","from","from","get_card","get_face_bytes","get_suit_bytes","into","into","new","selected","suit","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","value","vzip","vzip","Deck","borrow","borrow_mut","deck_vec","fmt","from","into","new_deck","try_from","try_into","type_id","vzip","Hand","_test_hand","alter_selected_card","borrow","borrow_mut","discard_selected","draw_card","draw_until_five_cards","fmt","from","hand_vec","into","new","try_from","try_into","type_id","vzip","UserInput","borrow","borrow_mut","card_selection","chk_parsed_double_input","chk_parsed_funds_input","chk_select_input","eq","from","get_user_input","input_string","insert_funds","into","ne","parse_input","player_lost_input","player_won_input","try_from","try_into","type_id","vzip","CardSelection","CreditsAvailable","Double","Evaluation","InsertCoin","MachineState","Wallet","Win","add_funds","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","chk_evaluation_for_win","chk_funds","credits","eq","evaluate_doubling","evaluate_hand","from","from","from","hand_type","hand_value","into","into","into","new","reduce_funds","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","CharHolder","add_face_down","borrow","borrow_mut","fmt","format_hand","format_string_to_chars","from","holder","into","new","print_hand_and_credits","print_insert_coin","print_tips","try_from","try_into","type_id","vzip"],"q":["lab1_grupp_9","","","","","","","lab1_grupp_9::card","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","lab1_grupp_9::deck","","","","","","","","","","","","lab1_grupp_9::hand","","","","","","","","","","","","","","","","","lab1_grupp_9::input","","","","","","","","","","","","","","","","","","","","","lab1_grupp_9::logic","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","lab1_grupp_9::utils","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","Card data type","enum for named card suits","","","","","","","","","","","","convert bytes to u8 integer","","","","","","create a card from given face and suit values","","","","","constructor","","","","","","","","","","","","","Deck data type","","","","","","","Generates 52 cards into a deck with 2 jokers","","","","","Hand data type","","","","","Discards any card that’s not selected","Draws a card from the Deck","","","","","","","","","","","","","","Player inputs to select or de-select a card","Check if the parsed input is within the allowed span of …","","Check if the parsed input is within the allowed span of …","","","","","Either adds funds or changes machine state depending on …","","","Try and parse the input","","Available options after a win scenario and changes states …","","","","","","","","Evaluation data type","","","Holds credits","","Add to playable credits","","","","","","","Check if the evaluation was a winning hand","Check if there are credits left","","","Evaluate if the machine or the player won the doubling …","Evaluate a Hand if it contains a winning poker hand","","","","","","","","","","Reduce credits by 1","","","","","","","","","","","","","Character holder data type","Formatted ASCII art for a face-down card","","","","Formats each card in the hand into a vector of characters …","iterate through each charachter in the string and push it …","","","","","Prints a 5 card hand in a row in the terminal","prints “Insert coin” ASCII-art in terminal","Matches each state with the correct tips string and prints …","","","",""],"i":[0,0,0,0,0,0,0,0,0,1,1,1,1,1,2,1,2,1,1,1,0,1,2,1,2,1,2,2,2,2,1,2,2,2,1,2,1,2,1,2,1,2,2,1,0,3,3,3,3,3,3,3,3,3,3,3,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,0,6,0,0,6,7,7,8,6,7,8,6,8,7,7,6,0,8,7,8,6,8,8,7,8,6,7,7,7,8,6,7,8,6,7,8,6,7,8,6,0,0,9,9,9,0,9,9,9,9,9,0,0,0,9,9,9,9],"f":[null,null,null,null,null,[[]],null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["cardsuit",4]],[[]],[[],["u8",15]],[[["cardsuit",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["u8",15],["u8",15]],["card",3]],[[],["vec",3,[["u8",15]]]],[[],["vec",3,[["u8",15]]]],[[]],[[]],[[]],null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],null,[[]],[[]],null,[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,[[]],[[["usize",15]]],[[]],[[]],[[]],[[["deck",3]]],[[["deck",3]]],[[["formatter",3]],["result",6]],[[]],null,[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,[[]],[[]],[[["hand",3],["vec",3],["machinestate",4],["wallet",3]]],[[],["option",4,[["usize",15]]]],[[],["option",4,[["usize",15]]]],[[],["option",4,[["usize",15]]]],[[["userinput",3]],["bool",15]],[[]],[[]],null,[[["wallet",3],["machinestate",4]]],[[]],[[["userinput",3]],["bool",15]],[[],["result",4,[["usize",15]]]],[[["wallet",3],["machinestate",4]],["bool",15]],[[["wallet",3],["machinestate",4],["usize",15],["bool",15]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,null,null,null,null,[[["usize",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[["machinestate",4]]],[[["machinestate",4]]],null,[[["machinestate",4]],["bool",15]],[[["hand",3],["usize",15],["usize",15]],["machinestate",4]],[[["hand",3]]],[[]],[[]],[[]],null,null,[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],null,[[["vec",3]]],[[]],[[]],[[["formatter",3]],["result",6]],[[["hand",3]],["vec",3,[["charholder",3]]]],[[["string",3]]],[[]],null,[[]],[[]],[[["vec",3],["wallet",3]]],[[]],[[["machinestate",4]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]]],"p":[[4,"CardSuit"],[3,"Card"],[3,"Deck"],[3,"Hand"],[3,"UserInput"],[4,"MachineState"],[3,"Wallet"],[3,"Evaluation"],[3,"CharHolder"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};