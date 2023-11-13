import { ComponentProps, DisplayComponent, EventBus, FSComponent, VNode } from "@microsoft/msfs-sdk"
import { getDefaultAppDomain } from "@navigraph/app"
import { CancelToken, navigraphRequest } from "navigraph/auth"
import { AuthService } from "../Services/AuthService"
import "./NavigraphLogin.css"

interface NavigraphLoginProps extends ComponentProps {
  bus: EventBus
}

export class NavigraphLogin extends DisplayComponent<NavigraphLoginProps> {
  private readonly textRef = FSComponent.createRef<HTMLDivElement>()
  private readonly navdataTextRef = FSComponent.createRef<HTMLDivElement>()
  private readonly buttonRef = FSComponent.createRef<HTMLButtonElement>()
  private readonly qrCodeRef = FSComponent.createRef<HTMLImageElement>()

  private cancelSource = CancelToken.source()

  private commBusListener: ViewListener.ViewListener

  constructor(props: NavigraphLoginProps) {
    super(props)

    this.commBusListener = RegisterViewListener("JS_LISTENER_COMM_BUS", () => {
      console.info("JS_LISTENER_COMM_BUS registered")
    })

    this.commBusListener.on("NavdataDownloaded", () => {
      console.info("WASM downloaded navdata")
      this.navdataTextRef.instance.textContent = "Navdata downloaded!"
    })

    this.commBusListener.on("UnzippedFilesRemaining", (jsonArgs: string) => {
      const args = JSON.parse(jsonArgs)
      console.info("WASM unzipping files", args)
      const percent = Math.round((args.unzipped / args.total) * 100)
      this.navdataTextRef.instance.textContent = `Unzipping files... ${percent}% complete`
    })
  }

  public render(): VNode {
    return (
      <div class="auth-container">
        <div ref={this.textRef} />
        <div ref={this.buttonRef} onClick={this.handleClick.bind(this)} class="login-button" />
        <div ref={this.navdataTextRef} />
        <img ref={this.qrCodeRef} class="qr-code" />
      </div>
    )
  }

  public onBeforeRender(): void {
    super.onBeforeRender()
  }

  public onAfterRender(node: VNode): void {
    super.onAfterRender(node)

    this.buttonRef.instance.addEventListener("click", () => this.handleClick().catch(e => console.error(e)))

    AuthService.user.sub(user => {
      if (user) {
        this.qrCodeRef.instance.src = ""
        this.qrCodeRef.instance.style.display = "none"
        this.buttonRef.instance.textContent = "Log out"
        this.textRef.instance.textContent = `Welcome, ${user.preferred_username}`

        this.handleLogin().catch(e => console.error(e))
      } else {
        this.buttonRef.instance.textContent = "Sign in"
        this.textRef.instance.textContent = "Not signed in"
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
          this.qrCodeRef.instance.src = `https://api.qrserver.com/v1/create-qr-code/?size=500x500&data=${p.verification_uri_complete}`
          this.qrCodeRef.instance.style.display = "block"
          console.info(p.verification_uri_complete)
        }
      }, this.cancelSource.token).catch(e => console.error("Failed to sign in!", e))
    }
  }

  private async handleLogin() {
    console.log("successful login, downloading navdata")
    this.navdataTextRef.instance.textContent = "Downloading navdata..."
    // leaving this here for now (messy) until the PR from the sdk is merged
    const result = await navigraphRequest
      .get(`https://fmsdata.api.${getDefaultAppDomain()}/v3/packages?format=avionics_v1`)
      .catch(e => console.error(e))
    const signedUrl = result.data[0].files[0].signed_url
    console.log("signed url", signedUrl)
    await this.commBusListener.call("COMM_BUS_WASM_CALLBACK", "DownloadNavdata", JSON.stringify({
      url: signedUrl
    }))
  }
}
