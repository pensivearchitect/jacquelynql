# encoding: UTF-8
# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# Note that this schema.rb definition is the authoritative source for your
# database schema. If you need to create the application database on another
# system, you should be using db:schema:load, not running all the migrations
# from scratch. The latter is a flawed and unsustainable approach (the more migrations
# you'll amass, the slower it'll run and the greater likelihood for issues).
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema.define(version: 20160513104847) do

  # These are extensions that must be enabled in order to support this database
  enable_extension "plpgsql"

  create_table "matches", force: :cascade do |t|
    t.string   "match_guid"
    t.string   "server_name"
    t.integer  "game_length"
    t.string   "game_type"
    t.datetime "created_at",  null: false
    t.datetime "updated_at",  null: false
  end

  create_table "matches_players", id: false, force: :cascade do |t|
    t.integer "match_id",  null: false
    t.integer "player_id", null: false
  end

  create_table "players", force: :cascade do |t|
    t.string   "name"
    t.string   "previous_names"
    t.string   "steam_id"
    t.datetime "created_at",     null: false
    t.datetime "updated_at",     null: false
  end

end
