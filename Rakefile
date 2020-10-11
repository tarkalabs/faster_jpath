require "rbconfig"
require "bundler/gem_tasks"
require "rspec/core/rake_task"

desc 'Preventative work'
task :tidy_up do
  sh 'cargo clean'
end

desc 'Build Rust extension'
task :build_lib do
  case RbConfig::CONFIG['host_os']
  when /darwin|mac os/
    sh 'cargo rustc --release -- -C link-args=-Wl,-undefined,dynamic_lookup'
  else
    sh 'cargo build --release'
  end
end

desc 'bundle install'
task :bundle_install do
  sh 'bundle install'
end

RSpec::Core::RakeTask.new(spec: [:tidy_up, :bundle_install, :build_lib])

task :default => :spec
