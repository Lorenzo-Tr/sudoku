<script lang="ts">
  import "./profile.css";
  import { siDiscord, siGoogle } from "simple-icons";

  export let userProfile = {
    name: "",
    handle: "",
    initials: "",
    streak: 0,
  };
  export let streakIcon = "";
</script>

<section class="profile-view">
  <div class="profile-banner">
    <span class="avatar large-avatar">{userProfile.initials}</span>
    <div class="profile-banner-copy">
      <span class="daily-kicker">Profile</span>
      <h1>{userProfile.name}</h1>
      <p>{userProfile.handle}</p>
    </div>
    <div class="profile-streak">
      {@html streakIcon}
      <span>{userProfile.streak}</span>
      <strong>day streak</strong>
    </div>
  </div>

  <div class="profile-settings">
    <section class="settings-section">
      <div class="settings-section-copy">
        <h2>Account details</h2>
        <p>Manage the sign-in details used for your Sudoku profile.</p>
      </div>
      <div class="settings-list">
        <div class="settings-row">
          <div>
            <strong>Email</strong>
            <span>lorenzo@example.com</span>
          </div>
          <button>Edit email</button>
        </div>
        <div class="settings-row">
          <div>
            <strong>Password</strong>
            <span>Last changed recently</span>
          </div>
          <button>Change password</button>
        </div>
      </div>
    </section>

    <section class="settings-section">
      <div class="settings-section-copy">
        <h2>Connected accounts</h2>
        <p>Link providers for faster sign-in and account recovery.</p>
      </div>
      <div class="settings-list">
        <div class="settings-row">
          <div>
            <span class="provider-title">
              <span class="provider-icon discord-icon" aria-hidden="true">
                <svg viewBox="0 0 24 24">
                  <path d={siDiscord.path} />
                </svg>
              </span>
              <strong>Discord</strong>
            </span>
            <span>Not linked</span>
          </div>
          <button>Link</button>
        </div>
        <div class="settings-row">
          <div>
            <span class="provider-title">
              <span class="provider-icon google-icon" aria-hidden="true">
                <svg viewBox="0 0 24 24">
                  <path d={siGoogle.path} />
                </svg>
              </span>
              <strong>Google</strong>
            </span>
            <span>Connected as lorenzo@gmail.com</span>
          </div>
          <button>Unlink</button>
        </div>
      </div>
    </section>

    <section class="settings-section danger-zone">
      <div class="settings-section-copy">
        <h2>Delete account</h2>
        <p>Permanently remove your profile, streak history, and daily archive stats.</p>
      </div>
      <button class="danger-button">Delete account</button>
    </section>
  </div>
</section>
