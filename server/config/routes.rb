Rails.application.routes.draw do
  namespace :api do
    resources :matches, constraints: { format: 'json' }
    resources :players, constraints: { format: 'json' }
  end
  mount_ember_app :frontend, to: '/'
end
