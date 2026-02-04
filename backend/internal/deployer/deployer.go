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

func NewDeployer() (*Deployer, error) {
	cli, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
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

func (d *Deployer) StreamLogs(ctx context.Context, containerID string) (io.ReadCloser, error) {
	return d.cli.ContainerLogs(ctx, containerID, types.ContainerLogsOptions{ShowStdout: true, ShowStderr: true, Follow: true})
}
