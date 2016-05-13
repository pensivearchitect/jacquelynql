module Api
  class MatchesController < ApplicationController
    before_action :set_match, only: [:show, :edit, :update, :destroy]

    # GET /matches
    # GET /matches.json
    def index
      @matches = Match.all
      render json: @matches
    end

    # GET /matches/1
    # GET /matches/1.json
    def show
      render json: @match
    end

    # GET /matches/new
    def new
      @match = Match.new
    end

    # GET /matches/1/edit
    def edit
    end

    # POST /matches
    # POST /matches.json
    def create
      @match = Match.new(match_params)

      if @match.save
        render json: @match, status: :created
      else
        render json: @match.errors, status: :unprocessable_entity
      end
    end

    # PATCH/PUT /matches/1
    # PATCH/PUT /matches/1.json
    def update
      if @match.update(match_params)
        head :no_content
      else
        render json: @match.errors, status: :unprocessable_entity
      end
    end

    # DELETE /matches/1
    # DELETE /matches/1.json
    def destroy
      @match.destroy
      respond_to do |format|
        head :no_content
      end
    end

    private
    # Use callbacks to share common setup or constraints between actions.
    def set_match
      @match = Match.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def match_params
      params.require(:match).permit(:players_id, :match_guid, :server_name, :game_length, :game_type)
    end
  end
end
