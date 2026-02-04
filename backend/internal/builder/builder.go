package builder
import (
	"fmt"
	"io"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)
type Builder struct{}
func NewBuilder() *Builder {
	return &Builder{}
}
func (b *Builder) Build(repoURL, targetCommit, appName, gitToken, buildCmd, startCmd, installCmd, runtime string, envVars map[string]string, logWriter io.Writer) (string, string, error) {
	workDir := filepath.Join("/tmp", "paas-builds", appName)
	if err := os.RemoveAll(workDir); err != nil {
		return "", "", fmt.Errorf("failed to clean work dir: %w", err)
	}
	if err := os.MkdirAll(workDir, 0755); err != nil {
		return "", "", fmt.Errorf("failed to create work dir: %w", err)
	}
	cloneURL := repoURL
	if gitToken != "" {
		u, err := url.Parse(repoURL)
		if err != nil {
			return "", "", fmt.Errorf("invalid repo url: %w", err)
		}
		u.User = url.UserPassword("oauth2", gitToken)
		cloneURL = u.String()
	}
	fmt.Fprintf(logWriter, ">>> Cloning repository %s...\n", repoURL)
	var cloneCmd *exec.Cmd
	if targetCommit != "" && targetCommit != "HEAD" && targetCommit != "MANUAL" && targetCommit != "WEBHOOK" {
		cloneCmd = exec.Command("git", "clone", "--filter=blob:none", cloneURL, ".")
	} else {
		cloneCmd = exec.Command("git", "clone", "--depth", "1", cloneURL, ".")
	}
	cloneCmd.Dir = workDir
	cloneCmd.Stdout = logWriter
	cloneCmd.Stderr = logWriter
	if err := cloneCmd.Run(); err != nil {
		return "", "", fmt.Errorf("git clone failed: %w", err)
	}
	if targetCommit != "" && targetCommit != "HEAD" && targetCommit != "MANUAL" && targetCommit != "WEBHOOK" {
		fmt.Fprintf(logWriter, ">>> Checking out commit %s...\n", targetCommit)
		checkoutCmd := exec.Command("git", "checkout", targetCommit)
		checkoutCmd.Dir = workDir
		checkoutCmd.Stdout = logWriter
		checkoutCmd.Stderr = logWriter
		if err := checkoutCmd.Run(); err != nil {
			return "", "", fmt.Errorf("git checkout failed: %w", err)
		}
	}
	commitCmd := exec.Command("git", "rev-parse", "HEAD")
	commitCmd.Dir = workDir
	commitHashBytes, err := commitCmd.Output()
	commitHash := ""
	if err == nil {
		commitHash = strings.TrimSpace(string(commitHashBytes))
		fmt.Fprintf(logWriter, ">>> Checked out commit: %s\n", commitHash)
	} else {
		fmt.Fprintf(logWriter, ">>> Failed to get commit hash: %v\n", err)
	}
	if runtime == "" {
		runtime = "nodejs"
	}
	var nixPkgs string
	var defaultInstall, defaultBuild, defaultStart string
	switch runtime {
	case "bun":
		nixPkgs = `["bun"]`
		defaultInstall = "bun install"
		defaultBuild = "bun run build"
		defaultStart = "bun run start"
	case "deno":
		nixPkgs = `["deno"]`
		defaultInstall = "deno cache"
		defaultBuild = "deno task build"
		defaultStart = "deno task start"
	case "pnpm":
		nixPkgs = `["nodejs_20", "pnpm"]`
		defaultInstall = "pnpm install"
		defaultBuild = "pnpm run build"
		defaultStart = "pnpm run start"
	default:
		nixPkgs = `["nodejs_20"]`
		defaultInstall = "npm ci --legacy-peer-deps || npm install --legacy-peer-deps"
		defaultBuild = "npm run build"
		defaultStart = "npm run start"
	}
	installStr := defaultInstall
	if installCmd != "" {
		installStr = installCmd
	}
	buildStr := defaultBuild
	if buildCmd != "" {
		buildStr = buildCmd
	}
	startStr := defaultStart
	if startCmd != "" {
		startStr = startCmd
	}
	nixpacksConfig := fmt.Sprintf(`
[phases.setup]
nixPkgs = %s
[phases.install]
cmds = ["%s"]
[phases.build]
cmds = ["%s"]
[start]
cmd = "%s"
`, nixPkgs, installStr, buildStr, startStr)
	if _, err := os.Stat(filepath.Join(workDir, "package.json")); err == nil {
		configPath := filepath.Join(workDir, "nixpacks.toml")
		if err := os.WriteFile(configPath, []byte(nixpacksConfig), 0644); err != nil {
			return "", "", fmt.Errorf("failed to write nixpacks.toml: %w", err)
		}
	}
	imageName := strings.ToLower(appName)
	fmt.Fprintf(logWriter, "\n>>> Starting Nixpacks build for %s...\n", imageName)
	args := []string{"build", ".", "--name", imageName, "--no-cache"}
	for k, v := range envVars {
		args = append(args, "--env", fmt.Sprintf("%s=%s", k, v))
	}
	nixCmd := exec.Command("nixpacks", args...)
	nixCmd.Dir = workDir
	nixCmd.Stdout = logWriter
	nixCmd.Stderr = logWriter
	nixCmd.Env = append(os.Environ(),
		"NIXPACKS_NO_CACHE=1",
	)
	if err := nixCmd.Run(); err != nil {
		return "", "", fmt.Errorf("nixpacks build failed: %w", err)
	}
	fmt.Fprintf(logWriter, "\n>>> Build successful!\n")
	return imageName, commitHash, nil
}
