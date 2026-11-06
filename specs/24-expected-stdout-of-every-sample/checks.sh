for kr in "$ROOT"/sample/*.kr; do
    if grep -q 'printf해요' "$kr"; then
        check "$(basename "${kr%.kr}")_has_out" file_exists "sample/$(basename "${kr%.kr}").out"
    fi
done
proto_is_a_sample() {
    (cd "$ROOT" && compgen -G 'sample/*[Pp]roto*.kr' > /dev/null && compgen -G 'sample/*[Pp]roto*.out' > /dev/null) || { echo "no sample/*proto*.kr with its .out"; return 1; }
}
check "proto_is_a_sample" proto_is_a_sample
