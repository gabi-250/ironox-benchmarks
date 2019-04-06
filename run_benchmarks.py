import argparse
import os
import subprocess
import sys
import math


RESULT_LINE = 'Mean'
REAL_LINE = 'real'
MEAN_IDX = 1
SIGMA_IDX = 2
BACKENDS = ['ironox', 'cranelift', 'llvm']
Z = 2.576


def parse_token(token):
    head, _, _ = token.partition('+')
    return head


def conf_interval(z, s, n):
    return z * s / math.sqrt(n)


def format_result(mean, error):
    return "${mean:.4f} \scriptstyle \pm{error:.4f}$".format(mean=mean,
                                                             error=error)

def get_env(rust_root, multitime, backend, clif_dir):
    return dict(os.environ,
                RUST_ROOT=rust_root,
                MULTITIME=multitime,
                RUST_BACKEND=backend,
                CLIF_DIR=clif_dir)


def mean_sigma(env, proj, num_obs, bench_type):
    cmd = "make bench_{} proj={} num_obs={}".format(bench_type, proj, num_obs)
    p = subprocess.run(cmd.split(),
                       stdout=subprocess.PIPE,
                       stderr=subprocess.PIPE,
                       env=env)
    if p.returncode:
        print('Failed to run {cmd}'.format(cmd=cmd))
        print('Output was:\n{}\n{}\nExiting...'.format(p.stdout.decode("utf-8"), p.stderr.decode("utf-8")),
                file=sys.stderr)
        sys.exit()
    else:
        output = p.stderr.decode()
        idx = output.find(RESULT_LINE)
        if idx == -1:
            print('Cannot find multitime output.', file=sys.stderr)
            print('Output was:\n{}\nExiting...'.format(output),
                  file=sys.stderr)
            sys.exit()
        for line in output[idx:].split('\n'):
            if line.startswith(REAL_LINE):
                line = line.split()
                mean = parse_token(line[MEAN_IDX])
                sigma = parse_token(line[SIGMA_IDX])
                return float(mean), float(sigma)
        sys.exit('Failed to parse multitime output. Exiting...')


def generate_table(template, bench_type, results):
    template_content = open(template, 'r').read()
    name, extension = os.path.splitext(template)
    name = '{}_{}_{}{}'.format(bench_type, name, 'result', extension)
    with open(name, 'w') as f:
        f.write(template_content.format(**results))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--rust',
                        help='the root of the rust project',
                        required=True)
    parser.add_argument('--multitime',
                        help='the path of the multitime binary',
                        required=True)
    parser.add_argument('--benchmarks',
                        help='the path of the benchmarks directory',
                        required=True)
    parser.add_argument('--template',
                        help='the latex template for the results table',
                        required=True)
    parser.add_argument('--clif',
                        help='the path of the cranelift project',
                        required=True)
    parser.add_argument('-n', '--num-obs',
                        help='the number of times to run the benchmarks',
                        type=int,
                        default=30)
    args = parser.parse_args()
    run_bench = []
    build_bench = []
    for d in os.listdir(args.benchmarks):
        if d.endswith('repeated'):
            build_bench.append(d)
        else:
            run_bench.append(d)
    run_results = {}
    build_results = {}
    for backend in BACKENDS:
        print('Benchmarking the %s backend' % (backend))
        env = get_env(args.rust, args.multitime, backend, args.clif)
        for bench in run_bench:
            print('\tBenchmark run: ', bench)
            mean, sigma = mean_sigma(env,
                                     bench,
                                     args.num_obs,
                                     'run')
            bench_code = '{}_{}'.format(backend, bench)
            error = conf_interval(Z, sigma, args.num_obs)
            result = format_result(mean, error)
            run_results[bench_code] = result
        for bench in build_bench:
            print('\tBenchmark build: ', bench)
            mean, sigma = mean_sigma(env,
                                     bench,
                                     args.num_obs,
                                     'build')
            bench = bench[:bench.find('_repeated')]
            bench_code = '{}_{}'.format(backend, bench)
            error = conf_interval(Z, sigma, args.num_obs)
            result = format_result(mean, error)
            build_results[bench_code] = result
    generate_table(args.template, 'run', run_results)
    generate_table(args.template, 'build', build_results)
