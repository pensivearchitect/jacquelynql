FactoryGirl.define do
  factory :match do
    players nil
    match_guid "MyString"
    server_name "MyString"
    game_length 1
    factory "MyString"
  end
  factory :player do
    name "MyString"
    previous_names "MyString"
    steam_id "MyString"
    matches nil
  end
end
