# ansible-playbooks

learning ansible

docker run --rm -v $(pwd):/ansible/playbooks quay.io/ansible/ansible-runner ansible-playbook /ansible/playbooks/hello_world.yml -i host.docker.internal