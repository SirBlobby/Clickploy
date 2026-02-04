package deployer
import (
	"context"
	"fmt"
	"io"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
)
type Deployer struct {
	cli *client.Client
}
func (d *Deployer) RemoveContainer(ctx context.Context, containerID string) error {
	return d.cli.ContainerRemove(ctx, containerID, types.ContainerRemoveOptions{Force: true})
}
func NewDeployer() (*Deployer, error) {
	cli, err := client.NewClientWithOpts(client.FromEnv, client.WithVersion("1.44"))
	if err != nil {
		return nil, fmt.Errorf("failed to create docker client: %w", err)
	}
	return &Deployer{cli: cli}, nil
}
func (d *Deployer) RunContainer(ctx context.Context, imageName, appName string, hostPort int, envVars []string) (string, error) {
	config := &container.Config{
		Image: imageName,
		ExposedPorts: nat.PortSet{
			"3000/tcp": struct{}{},
		},
		Env: envVars,
	}
	hostConfig := &container.HostConfig{
		PortBindings: nat.PortMap{
			"3000/tcp": []nat.PortBinding{
				{
					HostIP:   "0.0.0.0",
					HostPort: fmt.Sprintf("%d", hostPort),
				},
			},
		},
		RestartPolicy: container.RestartPolicy{
			Name: "unless-stopped",
		},
	}
	_ = d.cli.ContainerRemove(ctx, appName, types.ContainerRemoveOptions{Force: true})
	resp, err := d.cli.ContainerCreate(ctx, config, hostConfig, nil, nil, appName)
	if err != nil {
		return "", fmt.Errorf("failed to create container: %w", err)
	}
	if err := d.cli.ContainerStart(ctx, resp.ID, types.ContainerStartOptions{}); err != nil {
		return "", fmt.Errorf("failed to start container: %w", err)
	}
	return resp.ID, nil
}
func (d *Deployer) GetContainerEnv(ctx context.Context, containerID string) ([]string, error) {
	info, err := d.cli.ContainerInspect(ctx, containerID)
	if err != nil {
		return nil, err
	}
	return info.Config.Env, nil
}
func (d *Deployer) StreamLogs(ctx context.Context, containerID string) (io.ReadCloser, error) {
	return d.cli.ContainerLogs(ctx, containerID, types.ContainerLogsOptions{ShowStdout: true, ShowStderr: true, Follow: true})
}
func (d *Deployer) StartDatabaseContainer(ctx context.Context, image, name string, port int, volumePath string, envVars []string) (string, error) {
	_, err := d.cli.ImagePull(ctx, image, types.ImagePullOptions{})
	if err != nil {
		fmt.Printf("Failed to pull image %s: %v\n", image, err)
	}
	config := &container.Config{
		Image: image,
		ExposedPorts: nat.PortSet{
			"27017/tcp": struct{}{}, 
		},
		Env: envVars,
	}
	hostConfig := &container.HostConfig{
		PortBindings: nat.PortMap{
			"27017/tcp": []nat.PortBinding{
				{
					HostIP:   "0.0.0.0",
					HostPort: fmt.Sprintf("%d", port),
				},
			},
		},
		Binds: []string{
			fmt.Sprintf("%s:/data/db", volumePath),
		},
		RestartPolicy: container.RestartPolicy{
			Name: "unless-stopped",
		},
	}
	d.cli.ContainerRemove(ctx, name, types.ContainerRemoveOptions{Force: true})
	resp, err := d.cli.ContainerCreate(ctx, config, hostConfig, nil, nil, name)
	if err != nil {
		return "", fmt.Errorf("failed to create db container: %w", err)
	}
	if err := d.cli.ContainerStart(ctx, resp.ID, types.ContainerStartOptions{}); err != nil {
		return "", fmt.Errorf("failed to start db container: %w", err)
	}
	return resp.ID, nil
}
func (d *Deployer) ExecContainer(ctx context.Context, containerID string, cmd []string) error {
	execConfig := types.ExecConfig{
		Cmd:          cmd,
		AttachStdout: true,
		AttachStderr: true,
	}
	resp, err := d.cli.ContainerExecCreate(ctx, containerID, execConfig)
	if err != nil {
		return fmt.Errorf("failed to create exec: %w", err)
	}
	execID := resp.ID
	attachResp, err := d.cli.ContainerExecAttach(ctx, execID, types.ExecStartCheck{})
	if err != nil {
		return fmt.Errorf("failed to attach exec: %w", err)
	}
	defer attachResp.Close()
	io.Copy(io.Discard, attachResp.Reader)
	inspectResp, err := d.cli.ContainerExecInspect(ctx, execID)
	if err != nil {
		return fmt.Errorf("failed to inspect exec: %w", err)
	}
	if inspectResp.ExitCode != 0 {
		return fmt.Errorf("command exited with code %d", inspectResp.ExitCode)
	}
	return nil
}
func (d *Deployer) StopContainer(ctx context.Context, containerID string) error {
	return d.cli.ContainerStop(ctx, containerID, container.StopOptions{})
}
func (d *Deployer) StartContainer(ctx context.Context, containerID string) error {
	return d.cli.ContainerStart(ctx, containerID, types.ContainerStartOptions{})
}
func (d *Deployer) RestartContainer(ctx context.Context, containerID string) error {
	return d.cli.ContainerRestart(ctx, containerID, container.StopOptions{})
}
