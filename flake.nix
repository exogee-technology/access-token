{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/007ccf2f";
  };
  outputs = { self, nixpkgs }: 
  let 

    pkgs = import nixpkgs { 
      system = "x86_64-linux";
    };

    kyelewis = {
      name = "Kye Lewis";
      email = "kye.lewis@exogee.com";
      github = "kyelewis";
      githubId = 19619266;
    };

  in with pkgs; {

      devShells.x86_64-linux.default = mkShell {
        buildInputs = [ 
          cargo
          pkgconfig
          openssl
          openssl.dev
          python3
          xorg.libxcb.dev   # todo: linux only?
          rustfmt
        ];
      };

      packages.x86-64-linux.default = rustPlatform.buildRustPackage rec {
        pname = "tako";
        version = "0.1";
        cargoSha256 = lib.fakeSha256;
        cargoLock.lockFile = ./Cargo.lock;
  
        buildInputs = [ openssl openssl.dev ];
        nativeBuildInputs = [ pkgconfig python3 ];

        meta = with lib; {
          description = "A small CLI application, written in rust, that allows you to get an OKTA token for use in an app.";
          homepage = "https://github.com/exogee-technology/okta-token-cli";
          license = licenses.mit;
          maintainers = [
            kyelewis
          ];
        };

      };

    };
}
    
