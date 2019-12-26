%global debug_package %{nil}

Name:           escl-scan
Version:        0.1.0
Release:        1%{?dist}
Summary:        Utility for scanning with eSCL protocol writen in Rust

License:        GPLv3
URL:            https://github.com/ElXreno/escl-scan
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging
BuildRequires:  openssl-devel

%description
Utility for scanning with eSCL protocol writen in Rust.


%prep
%autosetup


%build
cargo build --release --locked


%install
cargo install --root=%{buildroot}%{_prefix} --path=. --locked
rm -f %{buildroot}%{_prefix}/.crates.toml


%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}



%changelog
* Thu Dec 26 2019 ElXreno <elxreno@gmail.com>
- Initial packaging
