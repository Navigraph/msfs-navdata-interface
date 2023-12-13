import { ComponentProps, DisplayComponent, EventBus, FSComponent, VNode } from "@microsoft/msfs-sdk"
import { CancelToken } from "navigraph/auth"
import { packages } from "../Lib/navigraph"
import { AuthService } from "../Services/AuthService"
import "./NavigraphLogin.css"
import { DownloadProgressPhase, NavigraphEventType, NavigraphNavdataInterface } from "@navigraph/navdata-sdk"
import { Dropdown } from "./Dropdown"

interface NavigraphLoginProps extends ComponentProps {
  bus: EventBus
}

export class NavigraphLogin extends DisplayComponent<NavigraphLoginProps> {
  private readonly textRef = FSComponent.createRef<HTMLDivElement>()
  private readonly navdataTextRef = FSComponent.createRef<HTMLDivElement>()
  private readonly loginButtonRef = FSComponent.createRef<HTMLButtonElement>()
  private readonly qrCodeRef = FSComponent.createRef<HTMLImageElement>()
  private readonly dropdownRef = FSComponent.createRef<Dropdown>()
  private readonly downloadButtonRef = FSComponent.createRef<HTMLButtonElement>()
  private readonly executeButtonRef = FSComponent.createRef<HTMLButtonElement>()
  private readonly inputRef = FSComponent.createRef<HTMLInputElement>()

  private cancelSource = CancelToken.source()

  private navdataInterface: NavigraphNavdataInterface

  constructor(props: NavigraphLoginProps) {
    super(props)

    this.navdataInterface = new NavigraphNavdataInterface()

    this.navdataInterface.onEvent(NavigraphEventType.DownloadProgress, data => {
      switch (data.phase) {
        case DownloadProgressPhase.Downloading:
          this.displayMessage("Downloading navdata...")
          break
        case DownloadProgressPhase.Cleaning:
          if (!data.deleted) return
          this.displayMessage(`Cleaning destination directory. ${data.deleted} files deleted so far`)
          break
        case DownloadProgressPhase.Extracting: {
          // Ensure non-null
          if (!data.unzipped || !data.total_to_unzip) return
          const percent = Math.round((data.unzipped / data.total_to_unzip) * 100)
          this.displayMessage(`Unzipping files... ${percent}% complete`)
          break
        }
      }
    })
  }

  public render(): VNode {
    return (
      <div class="auth-container">
        <div class="horizontal">
          <div class="vertical">
            <div ref={this.textRef}>Loading</div>
            <div ref={this.loginButtonRef} class="button" />
            <div ref={this.navdataTextRef} />
            <img ref={this.qrCodeRef} class="qr-code" />
          </div>
          <div class="vertical">
            <Dropdown ref={this.dropdownRef} />
            <div ref={this.downloadButtonRef} class="button">
              Download
            </div>
            <input
              ref={this.inputRef}
              type="text"
              id="sql"
              name="sql"
              value="SELECT * FROM tbl_airports"
              class="text-field"
            />
            <div ref={this.executeButtonRef} class="button">
              Execute SQL
            </div>
          </div>
        </div>
      </div>
    )
  }

  public onBeforeRender(): void {
    super.onBeforeRender()
  }

  public onAfterRender(node: VNode): void {
    super.onAfterRender(node)

    this.loginButtonRef.instance.addEventListener("click", () => this.handleClick().catch(e => this.displayError(e)))
    this.downloadButtonRef.instance.addEventListener("click", () => this.handleDownloadClick())

    this.executeButtonRef.instance.addEventListener("click", () => {
      console.time("query")
      this.navdataInterface
        .executeSql(this.inputRef.instance.value)
        .then(data => {
          console.timeEnd("query")
        })
        .catch(e => {
          console.error(e)
        })
    })

    AuthService.user.sub(user => {
      if (user) {
        this.qrCodeRef.instance.src = ""
        this.qrCodeRef.instance.style.display = "none"
        this.loginButtonRef.instance.textContent = "Log out"
        this.textRef.instance.textContent = `Welcome, ${user.preferred_username}`

        this.handleLogin()
      } else {
        this.loginButtonRef.instance.textContent = "Sign in"
        this.textRef.instance.textContent = "Not logged in"
      }
    }, true)
  }

  private async handleClick() {
    if (AuthService.getUser()) {
      await AuthService.signOut()
    } else {
      this.cancelSource = CancelToken.source() // Reset any previous cancellations
      AuthService.signIn(p => {
        if (p) {
          this.qrCodeRef.instance.src = `https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=${p.verification_uri_complete}`
          this.qrCodeRef.instance.style.display = "block"
          console.info(p.verification_uri_complete)
        }
      }, this.cancelSource.token).catch(e => this.displayError(e.message))
    }
  }

  private handleLogin() {
    // Let's display all of our packages
    packages
      .listPackages()
      .then(pkgs => {
        for (const pkg of pkgs) {
          this.dropdownRef.instance.addDropdownItem(pkg.format, pkg.format)
        }
      })
      .catch(e => console.error(e))
  }

  private handleDownloadClick() {
    packages
      .getPackage(this.dropdownRef.instance.getNavdataFormat() as string)
      .then(pkg => {
        if (this.navdataInterface.getIsInitialized()) {
          this.navdataInterface
            .downloadNavdata(pkg.file.url, pkg.format)
            .then(() => {
              console.info("WASM downloaded navdata")
              this.displayMessage("Downloaded!")
              this.displayMessage("Navdata downloaded")
              this.navdataInterface
                .setActiveDatabase(pkg.format)
                .then(() => {
                  console.info("WASM set active database")
                })
                .catch(e => {
                  this.displayError(e)
                })
            })
            .catch(e => {
              this.displayError(e)
            })
        } else {
          this.displayError("WASM not initialized")
        }
      })
      .catch(e => this.displayError(e))
  }

  private displayMessage(message: string) {
    this.navdataTextRef.instance.textContent = message
    this.navdataTextRef.instance.style.color = "white"
  }

  private displayError(error: string) {
    this.navdataTextRef.instance.textContent = error
    this.navdataTextRef.instance.style.color = "red"
  }
}
